import { Title } from 'solid-start';
import { getRandomNickname, nicknameIsTaken } from '~/ts/api-tauri';
import { Component, Match, Switch, createSignal } from 'solid-js';
import isAlphanumeric from 'validator/lib/isAlphanumeric';
import { BadgeVersion } from '~/components/badge_version';

async function getExampleNickame() {
	return getRandomNickname();
}

enum StateNickname {
	Empty,
	IsTaken,
	IsFree
}

async function checkNickname(input_nickname: string): Promise<StateNickname> {
	if (input_nickname.trim() == '') {
		return StateNickname.Empty;
	}

	if (await nicknameIsTaken(input_nickname)) {
		return StateNickname.IsTaken;
	}

	return StateNickname.IsFree;
}

const StateDivNickname: Component<{ state: StateNickname }> = (props) => {
	return (
		<div class="my-3 mt-2 text-sm">
			<Switch>
				<Match when={props.state === StateNickname.Empty}>
					<p class="text-accent">
						<strong>Ваш ник должен быть уникальным</strong>
					</p>
				</Match>
				<Match when={props.state === StateNickname.IsTaken}>
					<p class="text-error">
						<strong>Ваш ник уже занятый</strong>
					</p>
				</Match>
				<Match when={props.state === StateNickname.IsFree}>
					<p class="text-success">
						<strong>Ваш ник уникальный</strong>
					</p>
				</Match>
			</Switch>
		</div>
	);
};

const ButtonRegistrationAccount: Component<{ state: StateNickname }> = (props) => {
    // btn btn-secondary
    return (
        <div class="btn btn-secondary btn-disabled">
            <Switch>
                <Match when={props.state == StateNickname.IsFree}>
                    <button class="">Новый акканунт</button>
                </Match>
                <Match when={props.state != StateNickname.IsFree}>
                    <button class="btn-disabled">Новый акканунт</button>
                </Match>
            </Switch>
        </div>
    );
};

export default function Index() {
	let example_nickname = getExampleNickame();
	const [placeholderNickname, setPlaceholderNickname] = createSignal('example_nickname6416');
	const [stateNickname, setStateNickname] = createSignal(StateNickname.Empty);

	example_nickname.then((result) => {
		setPlaceholderNickname(result);
	});

	return (
		<main class="hero min-h-screen text-center">
			<div class="max-w-md">
				<Title>Регистрация</Title>

				<h1 class="py-6 text-5xl font-bold">Регистрация</h1>

				<input
					type="text"
					placeholder={'Пример ника: ' + placeholderNickname()}
					class="input input-bordered input-secondary w-full max-w-xs"
					onKeyPress={(e) => {
						if (!isAlphanumeric(e.key)) e.preventDefault();
					}}
					onInput={(e) => {
						checkNickname(e.target.value).then((state) => {
							setStateNickname(state);
						});
					}}
				/>

				<StateDivNickname state={stateNickname()} />

				<ButtonRegistrationAccount state={stateNickname()}/>
			</div>

            <BadgeVersion version="0.1.0-alpha.2" />
		</main>
	);
}
