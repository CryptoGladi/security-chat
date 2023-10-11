import { Component, Show, Suspense, createResource, createSignal, lazy } from 'solid-js';
import { FaSolidCircleCheck } from 'solid-icons/fa';
import { getCryptosForAccept } from '~/ts/api-tauri';
import { Loading } from '../small/loading';
import { Modal, ModalRootElement } from 'solid-js-modal';

// TODO perfectly-scrollable
// TODO Add solid-jest

let modalRef: any;

class ModalRef {
	public show() {
		this.ref.showModal();		
	}

	public ref!: ModalRootElement;
};

const SearchFriendModal: Component<{modal_ref: ModalRef}> = (props) => {
	return (
		<Modal ref={props.modal_ref.ref}>
    		<p>This is modal content</p>
  		</Modal>
	);
}

const DontHaveAddFriends: Component = () => {
	let modal_search_friend: ModalRef;

	return (
		<div class="hero h-full w-full text-center">
			<div class="flex flex-col items-center">
				<FaSolidCircleCheck size={100} class="self-center" />

				<div class="p-6">
					<p>У вас нет заявок в добавление друзья</p>
					<p>Может попробуйте найти себе друзей?</p>
				</div>

				<button class="btn btn-accent" onclick={() => {
					modal_search_friend.show();
				}}>Найти друга</button>
			</div>

			<SearchFriendModal modal_ref={modal_search_friend}/>
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
