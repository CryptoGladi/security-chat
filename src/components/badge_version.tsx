import { Component } from "solid-js";

export const BadgeVersion: Component<{ version: string }> = (props) => {
	return (
		<div class="left-0 top-0 flex space-x-2 px-1 py-1" style="position: absolute;">
			<div class="badge badge-secondary">{props.version}</div>
		</div>
	);
};