import { Component, Show, Suspense, createResource, createSignal, lazy } from 'solid-js';
import { FaSolidCircleCheck } from 'solid-icons/fa';
import { getCryptosForAccept } from '~/ts/api-tauri';
import { Loading } from '../small/loading';

const DontHaveAddFriends: Component = () => {
	return (
		<div class="hero h-full w-full text-center">
			<div class="flex flex-col items-center">
				<FaSolidCircleCheck size={100} class="self-center" />

				<div class="p-6">
					<p>У вас нет очереди в добавления в друзья</p>
					<p class="font-light opacity-10">Походу никто не хочет быть вашим другом...</p>
				</div>
			</div>
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

function sleep(time: number) {
	return new Promise((resolve) => setTimeout(resolve, time));
}

export const AddFriends: Component = () => {
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
