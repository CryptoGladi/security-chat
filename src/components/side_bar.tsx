import { Component, Index, Signal, createResource, createSignal, onMount } from 'solid-js';
import { BsPeople } from 'solid-icons/bs';
import { VsTerminal } from 'solid-icons/vs';
import { FaRegularUser } from 'solid-icons/fa';
import { useLocation, useNavigate } from 'solid-start';
import { AiOutlineHome } from 'solid-icons/ai';
import { getAllMyFriends } from '~/ts/api-tauri';
import _ from 'lodash';

class Item {
	icon: any;
	text!: string;
	navigate_path!: string;

	constructor(text: string, path: string, icon: any) {
		this.text = text;
		this.icon = icon;
		this.navigate_path = path;
	}
}

const Icon: Component<{ item: Item }> = (props) => {
	const location = useLocation();

	return (
		<li classList={{ 'border-accent border-l-2': location.pathname === props.item.navigate_path }}>
			<a class="tooltip tooltip-right rounded-none" data-tip={props.item.text}>
				{props.item.icon}
			</a>
		</li>
	);
};

function createItemForUser(nickname: string): Item {
	return new Item(
		`Пользователь: ${nickname}`,
		`/main_window/chat/${nickname}`,
		<FaRegularUser size={24} color="grey" />
	);
}

const StantardItems = [
	new Item('Главная страница', '/main', <AiOutlineHome size={24} color="grey" />),
	new Item('Друзья', '/main_window/add_friends', <BsPeople size={24} color="grey" />),
	new Item('Выполнить комманду', '/main_window/run_command', <VsTerminal size={24} color="grey" />)
];

const ItemForForEach: Component<{ item: Item }> = (props) => {
	const navigate = useNavigate();

	return (
		<div
			onclick={() => {
				navigate(props.item.navigate_path);
			}}
		>
			<Icon item={props.item} />
		</div>
	);
};

export const SideBar: Component = () => {
	let [users] = createResource(async () => {
		let userss = await getAllMyFriends();

		return _.map(userss, (e) => {
			return createItemForUser(e);
		});
	});

	return (
		<div class="h-[100svh] border-r-[1px]" style="border-color: grey;">
			<ul class="menu p-0">
				<Index each={StantardItems}>{(item, i) => <ItemForForEach item={item()} />}</Index>

				<hr style="border-color: grey;" />

				<Index each={users()}>{(item, i) => <ItemForForEach item={item()} />}</Index>
			</ul>
		</div>
	);
};
