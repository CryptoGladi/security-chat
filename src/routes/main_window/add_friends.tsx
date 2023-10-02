import { SideBar } from '~/components/side_bar';

export default function Index() {
	return (
		<main class="flex">
			<SideBar />

			<div class="tabs">
				<a class="tab tab-bordered">Запросы в друзья</a>
				<a class="tab tab-active tab-bordered">ыы</a>
				<a class="tab tab-bordered">Tab 3</a>
			</div>
		</main>
	);
}
