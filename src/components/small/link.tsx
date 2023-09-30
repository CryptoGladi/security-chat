import { Component } from 'solid-js';
import { openThat } from '~/ts/api-tauri';

export const Link: Component<{ text: string; link: string }> = (props) => {
	return (
		<a
			class="inline-flex cursor-pointer font-medium text-accent-focus hover:underline"
			onclick={() => {
				openThat(props.link);
			}}
		>
			{props.text}
		</a>
	);
};
