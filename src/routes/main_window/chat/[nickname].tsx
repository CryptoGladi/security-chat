import { SideBar } from '~/components/side_bar';
import { useParams } from 'solid-start';
import { Component, For, Signal, createResource, createSignal } from 'solid-js';
import '~/styles/scrollbar.scss';
import { sendMessage } from '~/ts/api-tauri';

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

//@ts-ignore
let [messages, setMessages]: Signal<Message[]> = createSignal([]);

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

export default function Index() {
	const params = useParams<{ nickname: string }>();

	const messagess = createResource(async () => {});

	let inputMessage: HTMLTextAreaElement;
	let scrollableDiv: HTMLDivElement;

	return (
		<main class="flex h-screen w-full">
			<SideBar />

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
		</main>
	);
}
