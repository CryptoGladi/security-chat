import { Component, For, Show, Suspense, createResource, createSignal, lazy } from 'solid-js';
import { FaSolidCircleCheck } from 'solid-icons/fa';
import { addCrypto, getCryptosForAccept } from '~/ts/api-tauri';
import { Loading } from '../small/loading';
import { HiSolidXMark } from 'solid-icons/hi';
import { HiSolidPlus } from 'solid-icons/hi';
import '~/styles/scrollbar.scss';

const HaveAddFriends: Component<{ crypto_for_accept: string[] }> = (props) => {
	return (
		<div class="h-full w-full">
			<div class="scrollbar">
				<For each={props.crypto_for_accept}>
					{(item, index) => (
						<div class="m-2 flex items-center bg-neutral p-3">
							<p>{item}</p>

							<div class="flex w-full justify-end space-x-2">
								<button
									class="btn btn-circle btn-neutral btn-active btn-sm"
									onclick={() => {
										addCrypto(item).then(() => {
											window.location.reload();
										});
									}}
								>
									<HiSolidPlus size={20} color="#52fa7c" />
								</button>

								<button
									class="btn btn-circle btn-neutral  btn-active btn-sm"
									onclick={() => {
										// TODO
									}}
								>
									<HiSolidXMark size={20} color="#ff5757" />
								</button>
							</div>
						</div>
					)}
				</For>
			</div>
		</div>
	);
};

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

			<Show when={props.crypto_for_accept.length !== 0}>
				<HaveAddFriends crypto_for_accept={props.crypto_for_accept} />
			</Show>
		</div>
	);
};

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
