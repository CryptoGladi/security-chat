import { Component } from 'solid-js';
import { SolidTyper } from 'solid-typer';

export const Loading: Component = (p) => {
	return (
		<div class="hero text-center">
			<div>
				<span class="loading loading-spinner text-primary"></span>
				<p>
					<SolidTyper
						text={['Пожалуйста подождите...']}
						backspaceSpeed={30}
						typingSpeed={100}
						loop={true}
					/>
				</p>
			</div>
		</div>
	);
};
