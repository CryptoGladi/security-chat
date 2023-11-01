import { SideBar } from '~/components/side_bar';
import { useParams } from 'solid-start';
import { Component, For, Signal, createResource, createSignal, onMount } from 'solid-js';
import '~/styles/scrollbar.scss';
import { MessageInfo, getMessageByNickname, sendMessage } from '~/ts/api-tauri';
import _ from 'lodash';
import { Loading } from '~/components/small/loading';

class Message {
	constructor(text: string, type: TypeBubble) {
		this.text = text;
		this.type = type;
	}

	text!: string;
	type!: TypeBubble;
}

enum TypeBubble {
	ChatStart,
	ChatEnd
}

const MessageBubble: Component<{ message: Message }> = (props) => {
	return (
		<div
			class="chat"
			classList={{
				'chat-start': props.message.type === TypeBubble.ChatStart,
				'chat-end': props.message.type === TypeBubble.ChatEnd
			}}
		>
			<div class="chat-bubble">{props.message.text}</div>
		</div>
	);
};

function getTypeBubble(my_nickname: string, message: MessageInfo): TypeBubble {
	return message.sender === my_nickname ? TypeBubble.ChatStart : TypeBubble.ChatEnd;
}

const ShowData: Component<{ messages: MessageInfo[] }> = (props) => {
	const params = useParams<{ nickname: string }>();
	let inputMessage: HTMLTextAreaElement;
	let scrollableDiv: HTMLDivElement;

	let [messages, setMessages]: Signal<Message[]> = createSignal(
		_.reverse(
			_.map(props.messages, (e) => {
				let type = getTypeBubble(params.nickname, e);
				return new Message(e.body.text, type);
			})
		)
	);

	onMount(() => {
		scrollableDiv.scrollTop = scrollableDiv.scrollHeight;
	});

	return (
		<div class="flex h-full w-full flex-col">
			<div class="w-full flex-none items-center border-b-[1px] p-2" style="border-color: grey;">
				<p class="font-bold text-secondary">{params.nickname}</p>
			</div>

			<div
				// @ts-ignore
				ref={scrollableDiv}
				class="flex-1 scrollbar"
				style={{ position: 'relative' }}
			>
				<For each={messages()}>
					{(message, index) => <MessageBubble message={message}></MessageBubble>}
				</For>
			</div>

			<div class="flex h-auto flex-none flex-row border-t-[1px]" style="border-color: grey;">
				<textarea
					class="textarea textarea-ghost flex-1 focus:outline-none"
					placeholder="Для отправки сообщения нажмите Enter"
					maxlength={200}
					// @ts-ignore
					ref={inputMessage}
					onkeydown={(e) => {
						if (e.key === 'Enter' && !e.shiftKey && inputMessage.value.trim() !== '') {
							setMessages((a) => [...a, new Message(inputMessage.value, TypeBubble.ChatEnd)]);
							sendMessage(params.nickname, inputMessage.value);

							inputMessage.value = '';
							scrollableDiv.scrollTop = scrollableDiv.scrollHeight;

							e.preventDefault();
						}
					}}
				></textarea>
			</div>
		</div>
	);
};

export default function Index() {
	const params = useParams<{ nickname: string }>();

	const [resource_messages] = createResource(async () => {
		console.error('LOADING');
		let i = await getMessageByNickname(params.nickname);
		console.error(i);
		return i;
	});

	return (
		<main class="flex h-screen w-full">
			<SideBar />

			{resource_messages.loading ? <Loading /> : <ShowData messages={resource_messages()!} />}
		</main>
	);
}
