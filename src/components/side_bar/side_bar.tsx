import { Component, Index } from 'solid-js';
import './side_bar.css';
import { BsPeopleFill } from 'solid-icons/bs';
import { BsPeople } from 'solid-icons/bs';
import { VsDebugStart, VsPlay, VsTerminal } from 'solid-icons/vs';

class Item {
	icon: any;
	text!: string;

	constructor(text: string, icon: any) {
		this.text = text;
		this.icon = icon;
	}
}

const Icon: Component<{ item: Item }> = (props) => {
	// TODO SolidIcon
	return (
		<li>
			<a class="tooltip tooltip-right rounded-none" data-tip={props.item.text}>
				{props.item.icon}
			</a>
		</li>
	);
};

const StantardItems = [
	new Item('Запросы в друзья', <BsPeople size={24} color="grey" />),
	new Item('Выполнить комманду', <VsTerminal size={24} color="grey" />)
];

export const SideBar: Component = () => {
	return (
		<ul class="menu p-0">
			<Index each={StantardItems}>{(item, i) => <Icon item={item()} />}</Index>

			<Icon item={new Item('Пользователь {dd}', <VsTerminal size={24} color="grey" />)} />
		</ul>
	);
};
