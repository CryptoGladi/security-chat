import { Component } from 'solid-js';
import { openThat } from '~/ts/api-tauri';

export const Link: Component<{ text: string; link: string }> = (pros) => {
	return (
		<a
			class="inline-flex cursor-pointer font-medium text-accent-focus hover:underline"
			onclick={() => {
				openThat(pros.link);
			}}
		>
			{pros.text}
		</a>
	);
};
