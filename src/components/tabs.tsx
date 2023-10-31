import { Component, For, Setter, createSignal } from 'solid-js';

export class Tab {
	name!: string;
	compoment!: Element;

	constructor(name: string, component: Component) {
		this.name = name;
		this.compoment = component as unknown as Element;
	}
}

const ComponentTab: Component<{
	tab: Tab;
	my_index: number;
	currect_index: number;
	index_setter: Setter<number>;
}> = (props) => {
	return (
		<a
			class="tab tab-bordered flex-1"
			classList={{ 'tab-active': props.my_index === props.currect_index }}
			onclick={() => {
				props.index_setter(props.my_index);
			}}
		>
			{props.tab.name}
		</a>
	);
};

export const Tabs: Component<{ tabs: Tab[]; default_index: number }> = (props) => {
	let [currectIndex, setCurrectIndex] = createSignal(props.default_index);

	return (
		<div class="w-full flex-row overflow-hidden">
			<div class="flex">
				<For each={props.tabs}>
					{(tab, index) => (
						<ComponentTab
							tab={tab}
							my_index={index()}
							currect_index={currectIndex()}
							index_setter={setCurrectIndex}
						/>
					)}
				</For>
			</div>

			<div class="h-full">{props.tabs[currectIndex()].compoment}</div>
		</div>
	);
};
