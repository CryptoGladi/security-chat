import { FaSolidFaceFrownOpen } from "solid-icons/fa";
import { Component, Show } from "solid-js";
import { Loading } from '~/components/small/loading';
import { getCryptosForAccept } from '~/ts/api-tauri';

const NotFriends: Component = () => {
	return (
		<div class="h-full w-full hero text-center">
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

const HaveFriends: Component = () => {
    // TODO

    return (
        <div>
            TODO
        </div>
    );
}

export const AllFriends: Component = () => {
    let have_friends = false;
    return (
        <Show when={!have_friends}>
            <NotFriends/>
        </Show>
    );
}