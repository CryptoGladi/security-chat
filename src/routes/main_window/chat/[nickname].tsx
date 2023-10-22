import { SideBar } from '~/components/side_bar';
import { useParams } from 'solid-start';

export default function Index() {
	const params = useParams<{ nickname: string }>();

	return (
		<main class="flex">
			<SideBar />

			<div class="h-full w-full">
				<div class="flex h-10 w-full items-center border-b-[1px] p-2" style="border-color: grey;">
					<p class="font-bold text-secondary">{params.nickname}</p>
				</div>

				<div>
					<p>messages</p>
				</div>

				<div>
					<p>messager</p>
				</div>
			</div>
		</main>
	);
}
