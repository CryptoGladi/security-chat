import { Component, Show, createResource, createSignal } from 'solid-js';
import { FaSolidCircleCheck } from 'solid-icons/fa';
import { getCryptosForAccept, nicknameIsTaken, sendCrypto } from '~/ts/api-tauri';
import { Loading } from '../small/loading';
import { FaSolidCheck } from 'solid-icons/fa';
import { Toast } from '~/ts/custom-toast';

// TODO perfectly-scrollable
// TODO Add solid-jest

enum Status {
	Ok,
	NicknameNotFound
}

async function rawSendRequest(nickname: string): Promise<Status> {
	if (!(await nicknameIsTaken(nickname))) return Status.NicknameNotFound;

	await sendCrypto(nickname);

	return Status.Ok;
}

async function sendRequest(nickname: string) {
	let status = await rawSendRequest(nickname);
	
	switch (status) {
		case Status.Ok: Toast.success("");
		case Status.NicknameNotFound: Toast.error("Пользователь не найден");
	}
}

const AddFriendModal: Component = () => {
	let [buttonIsEnable, setButtonIsEnable] = createSignal(false);
	let fd = '3232';

	return (
		<div>
			<h3 class="text-lg font-bold">Добавить друга</h3>

			<div class="join py-6">
				<input
					class="input join-item input-bordered"
					placeholder="Ник"
					oninput={(e) => {
						setButtonIsEnable(e.target.value.length != 0);
						Toast.error('Вы отправили запрос пользователю');
					}}
				/>

				<button
					class="btn btn-accent join-item rounded-r-full"
					classList={{ 'btn-disabled': !buttonIsEnable() }}
				>
					<FaSolidCheck size={20} />
				</button>
			</div>
		</div>
	);
};

const DontHaveAddFriends: Component = () => {
	let fdd: HTMLDialogElement | ((el: HTMLDialogElement) => void) | undefined;

	return (
		<div class="hero h-full w-full text-center">
			<div class="flex flex-col items-center">
				<FaSolidCircleCheck size={100} class="self-center" />

				<div class="p-6">
					<p>У вас нет заявок в добавление друзья</p>
					<p>Может попробуйте найти себе друзей?</p>
				</div>

				<button
					class="btn btn-accent"
					onclick={() => {
						// @ts-ignore
						fdd?.showModal();
					}}
				>
					Найти друга
				</button>
			</div>

			<dialog ref={fdd} class="modal">
				<div class="modal-box">
					<AddFriendModal />
				</div>
				<form method="dialog" class="modal-backdrop">
					<button>close</button>
				</form>
			</dialog>
		</div>
	);
};

const ShowData: Component<{ crypto_for_accept: string[] | undefined }> = (props) => {
	if (props.crypto_for_accept == undefined) {
		throw new Error('props.crypto_for_accept == undefined');
	}

	return (
		<div class="h-full">
			<Show when={props.crypto_for_accept.length === 0}>
				<DontHaveAddFriends />
			</Show>
		</div>
	);
};

export const RequestsFriends: Component = () => {
	const [crypto_for_accept] = createResource(async () => {
		return await getCryptosForAccept();
	});

	return (
		<div class="h-full">
			{crypto_for_accept.loading ? (
				<Loading />
			) : (
				<ShowData crypto_for_accept={crypto_for_accept()} />
			)}
		</div>
	);
};
