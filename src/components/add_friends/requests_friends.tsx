import { Component, For, Show, Switch, createResource, createSignal } from 'solid-js';
import { FaSolidCircleCheck } from 'solid-icons/fa';
import { nicknameIsTaken, sendCrypto, getOrderAddingCrypto } from '~/ts/api-tauri';
import { Loading } from '../small/loading';
import { FaSolidCheck } from 'solid-icons/fa';
import { Toast } from '~/ts/custom-toast';
import '~/styles/scrollbar.scss';
import { AiOutlinePlus } from 'solid-icons/ai';

enum Status {
	Ok,
	NicknameNotFound
}

async function rawSendRequest(nickname: string): Promise<Status> {
	if (!(await nicknameIsTaken(nickname))) return Status.NicknameNotFound;

	await sendCrypto(nickname);

	return Status.Ok;
}

async function toastSendRequest(nickname: string) {
	switch (await rawSendRequest(nickname)) {
		case Status.Ok: {
			Toast.success('Заявка успешно отправлена');
			break;
		}
		case Status.NicknameNotFound: {
			Toast.error('Пользователь не найден!');
			break;
		}
	}
}

const AddFriendModal: Component<{ modal: HTMLDialogElement }> = (props) => {
	const [buttonIsEnable, setButtonIsEnable] = createSignal(false);
	const [inputNickname, setInputNickname] = createSignal('');

	return (
		<div>
			<h3 class="text-lg font-bold">Добавить друга</h3>

			<div class="join py-6">
				<input
					class="input join-item input-bordered focus:outline-none"
					placeholder="Ник"
					oninput={(e) => {
						setButtonIsEnable(e.target.value.length != 0);
						setInputNickname(e.target.value);
					}}
				/>

				<button
					class="btn btn-accent join-item rounded-r-full"
					classList={{ 'btn-disabled': !buttonIsEnable() }}
					onclick={() => {
						toastSendRequest(inputNickname());
						props.modal.close();
					}}
				>
					<FaSolidCheck size={20} />
				</button>
			</div>
		</div>
	);
};

const DontHaveAddFriends: Component = () => {
	let modal_add_friend: HTMLDialogElement | undefined;

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
						modal_add_friend?.showModal();
					}}
				>
					Найти друга
				</button>
			</div>

			<dialog ref={modal_add_friend} class="modal">
				<div class="modal-box">
					<AddFriendModal modal={modal_add_friend!} />
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

	let modal_add_friend: HTMLDialogElement | undefined;

	return (
		<div class="h-full">
			<Show when={props.crypto_for_accept.length === 0}>
				<DontHaveAddFriends />
			</Show>

			<Show when={props.crypto_for_accept.length !== 0}>
				<div class="h-full w-full scrollbar">
					<For each={props.crypto_for_accept}>
						{(item, index) => (
							<div class="m-2 flex items-center bg-neutral p-3">
								<p>{item}</p>
							</div>
						)}
					</For>
				</div>

				<div class="tooltip tooltip-left absolute bottom-0 right-0" data-tip="Отправить запрос">
					<button
						class="btn btn-accent m-2 rounded-full"
						onclick={() => {
							modal_add_friend?.showModal();
						}}
					>
						<AiOutlinePlus size="16" color="black" />
					</button>
				</div>
			</Show>

			<dialog ref={modal_add_friend} class="modal">
				<div class="modal-box">
					<AddFriendModal modal={modal_add_friend!} />
				</div>
				<form method="dialog" class="modal-backdrop">
					<button>close</button>
				</form>
			</dialog>
		</div>
	);
};

export const RequestsFriends: Component = () => {
	const [orderAddingCrypto] = createResource(async () => {
		return await getOrderAddingCrypto();
	});

	return (
		<div class="h-full">
			{orderAddingCrypto.loading ? (
				<Loading />
			) : (
				<ShowData crypto_for_accept={orderAddingCrypto()} />
			)}
		</div>
	);
};
