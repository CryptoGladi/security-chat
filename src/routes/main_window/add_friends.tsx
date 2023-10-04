import { Component, Show, Suspense, createResource, createSignal, lazy } from 'solid-js';
import { SideBar } from '~/components/side_bar';
import { Loading } from '~/components/small/loading';
import { getCryptosForAccept } from '~/ts/api-tauri';
import { FaSolidFaceFrownOpen } from 'solid-icons/fa'

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

export default function Index() {
	const [data] = createResource(async () => {
		return await getCryptosForAccept();
	})


	

	return (
		<main class="flex">
			<SideBar />

			<div>
				
			</div>
			<div class="tabs flex-row">
  				<a class="tab tab-bordered">Tab 1</a> 
  				<a class="tab tab-bordered tab-active">Tab 2</a> 
  				<a class="tab tab-bordered">Tab 3</a>
			</div>



			<Suspense fallback={<Loading />}>
				<Show when={data()?.length === 0}>
					<NotFriends/>
				</Show>
				
				<Show when={data()?.length !== 0}>
					<p>s</p>
				</Show>
      		</Suspense>
		</main>
	);
}
