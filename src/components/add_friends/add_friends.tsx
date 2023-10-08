import { Component, Show, Suspense, createResource, lazy } from 'solid-js';
import { FaSolidCircleCheck } from 'solid-icons/fa';
import { getCryptosForAccept } from '~/ts/api-tauri';
import { Loading } from '../small/loading';
import { lazily } from 'solidjs-lazily';

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

const HaveAddFriends: Component = () => {
	return <div></div>;
};

const { MyComponent } = lazily(async () => import('./async/list_friends'));

export const AddFriends: Component = () => {
	const [crypto_for_accept] = createResource(async () => {
		return await getCryptosForAccept();
	});

    // TODO https://codesandbox.io/embed/mystifying-roentgen-2o4wmxj9zy?codemirror=1
	return (
		<Suspense fallback={<p>Loading...</p>}>
			<MyComponent lll={crypto_for_accept}></MyComponent>
		</Suspense>
	);
};
