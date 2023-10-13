import { Component, Show, Suspense, createResource, createSignal, lazy } from 'solid-js';
import { FaSolidCircleCheck } from 'solid-icons/fa';
import { getCryptosForAccept } from '~/ts/api-tauri';
import { Loading } from '../small/loading';
import { FaSolidCheck } from 'solid-icons/fa'

// TODO perfectly-scrollable
// TODO Add solid-jest

const AddFriendModal: Component = () => {
	let [buttonIsEnable, setButtonIsEnable] = createSignal(false);

	return (
		<div>
			<h3 class="text-lg font-bold">Добавить друга</h3>

			<div class="join py-6">
	  			<input class="input input-bordered join-item" placeholder="Ник" oninput={(e) => {
					setButtonIsEnable(e.target.value.length != 0)
					// TODO solid-toast
				}}/>

	  			<button class="btn join-item rounded-r-full" classList={{"btn-disabled": !buttonIsEnable()}}>
	  				<FaSolidCheck size={20}/>
				</button>
			</div>
		</div>
	);
}

const DontHaveAddFriends: Component = () => {
	return (
		<div class="hero h-full w-full text-center">
			<div class="flex flex-col items-center">
				<FaSolidCircleCheck size={100} class="self-center" />

				<div class="p-6">
					<p>У вас нет заявок в добавление друзья</p>
					<p>Может попробуйте найти себе друзей?</p>
				</div>

				<button class="btn btn-accent" onclick="my_modal_1.showModal()">
					Найти друга
				</button>
			</div>

			<dialog id="my_modal_1" class="modal">
				<div class="modal-box">
					<AddFriendModal/>
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
