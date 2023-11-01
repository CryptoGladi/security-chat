import { Component, createSignal } from 'solid-js';
import { getVersionApp } from '~/ts/api-tauri';

export const BadgeVersion: Component = (props) => {
	const [version, setVersion] = createSignal('version');

	getVersionApp().then((e) => {
		setVersion(e);
	});

	return (
		<div class="left-0 top-0 flex space-x-2 px-1 py-1" style="position: absolute;">
			<div class="badge badge-secondary">{version()}</div>
		</div>
	);
};
