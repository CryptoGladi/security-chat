import { Component } from 'solid-js';

class DoneStep {
	readme = false;
	registration = false;
}

const Step: Component<{ is_activate: boolean; text: string }> = (props) => {
	return <li class={`step ${props.is_activate ? 'step-primary' : ''}`}>{props.text}</li>;
};

const Steps: Component<{ current_step: DoneStep }> = (props) => {
	return (
		<ul class="steps absolute inset-x-0 bottom-0 py-3">
			<Step is_activate={props.current_step.readme} text="README" />
			<Step is_activate={props.current_step.registration} text="Регистрация" />
		</ul>
	);
};

export default function Index() {
	let done_step = new DoneStep();

	return <Steps current_step={done_step} />;
}
