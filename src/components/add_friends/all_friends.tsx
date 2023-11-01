import { FaSolidFaceFrownOpen } from 'solid-icons/fa';
import { Component, For, Show, createResource } from 'solid-js';
import { Loading } from '~/components/small/loading';
import { getAllMyFriends, getCryptosForAccept } from '~/ts/api-tauri';

const NotFriends: Component = () => {
	return (
		<div class="hero h-full w-full text-center">
			<div class="flex flex-col items-center">
				<FaSolidFaceFrownOpen size={100} class="self-center" />

				<div class="p-6">
					<p>О нет! У вас нет друзей</p>
					<p class="text-accent">
						Может вам нужно их <span class="text-error">найти</span>?
					</p>
				</div>
			</div>
		</div>
	);
};

const HaveFriends: Component<{ all_my_friends: string[] }> = (props) => {
	// TODO

	return (
		<div>
			<For each={props.all_my_friends}>
				{(item, index) => (
					<div class="m-2 flex items-center bg-neutral p-3">
						<p>{item}</p>
					</div>
				)}
			</For>
		</div>
	);
};

const ShowData: Component<{ all_my_friends: string[] | undefined }> = (props) => {
	if (props.all_my_friends == undefined) {
		throw new Error('props.all_my_friends == undefined');
	}

	return (
		<div class="h-full w-full">
			<Show when={props.all_my_friends.length === 0}>
				<NotFriends />
			</Show>

			<Show when={props.all_my_friends.length !== 0}>
				<HaveFriends all_my_friends={props.all_my_friends} />
			</Show>
		</div>
	);
};

export const AllFriends: Component = () => {
	const [allMyFriends] = createResource(async () => {
		return await getAllMyFriends();
	});

	return (
		<div class="h-full">
			{allMyFriends.loading ? <Loading /> : <ShowData all_my_friends={allMyFriends()} />}
		</div>
	);
};
