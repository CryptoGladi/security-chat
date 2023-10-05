import { Component, Show, Suspense, createResource, createSignal, lazy } from 'solid-js';
import { SideBar } from '~/components/side_bar';
import { Loading } from '~/components/small/loading';
import { getCryptosForAccept } from '~/ts/api-tauri';
import { FaSolidFaceFrownOpen } from 'solid-icons/fa'
import { Tabs, Tab } from '~/components/tabs';

export const NotFriends: Component = () => {
	return (
		<div class="hero text-center">
			<div class='flex flex-col items-center'>
				<FaSolidFaceFrownOpen size={100} class="self-center" />

				<div class="p-6">
					<p>О нет! У вас нет друзей</p>
					<p class="text-accent">Может вам нужно их <span class="text-error">найти</span>?</p>
				</div>
			</div>
		</div>
	);
};

const ConstTabs: Tab[] = [new Tab('Ваши друзья', NotFriends), new Tab('Заявки', NotFriends), new Tab('Запросы', NotFriends)];

export default function Index() {
	const [data] = createResource(async () => {
		return await getCryptosForAccept();
	})


	return (
		<main class="flex">
			<SideBar />
			<Tabs tabs={ConstTabs} default_index={0}/>	
		</main>
	);
}
