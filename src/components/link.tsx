import { Component } from 'solid-js';
import { open_that } from '~/ts/api-tauri';

export const Link: Component<{text: string, link: string}> = (pros) => {
	return (
		<a
			class="inline-flex cursor-pointer font-medium text-accent-focus hover:underline"
			onclick={() => {
				open_that(pros.link);
			}}
		>
			{pros.text}
		</a>
	);
};
