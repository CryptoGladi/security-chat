import { SideBar } from '~/components/side_bar';
import { useParams } from 'solid-start';
import { Component } from 'solid-js';

enum TypeBubble {
	ChatStart,
	ChatEnd
}

const MessageBubble: Component<{ text: string; type: TypeBubble }> = (props) => {
	return (
		<div
			class="chat"
			classList={{
				'chat-start': props.type === TypeBubble.ChatStart,
				'chat-end': props.type === TypeBubble.ChatEnd
			}}
		>
			<div class="chat-bubble">{props.text}</div>
		</div>
	);
};

export default function Index() {
	const params = useParams<{ nickname: string }>();

	return (
		<main class="flex h-screen w-full">
			<SideBar />

			<div class="flex h-full w-full flex-col">
				<div class="w-full flex-none items-center border-b-[1px] p-2" style="border-color: grey;">
					<p class="font-bold text-secondary">{params.nickname}</p>
				</div>

				<div class="flex-1">
					<MessageBubble text={'Рандомный текст'} type={TypeBubble.ChatStart} />

					<MessageBubble text={'Ответ на рандомных текст'} type={TypeBubble.ChatEnd} />
				</div>

				<div class="h-10 flex-none border-t-[1px]" style="border-color: grey;">
					<p>messager</p>
				</div>
			</div>
		</main>
	);
}
