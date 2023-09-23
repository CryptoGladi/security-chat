import { Title } from "solid-start";
import { get_random_nickname } from "~/ts/api-tauri";
import { createSignal } from "solid-js";

async function get_example_nickame() {
    return get_random_nickname();
}

export default function Index() {
    let example_nickname = get_example_nickame();
    const [placeholderNickname, setPlaceholderNickname] = createSignal("ss");

    example_nickname.then((result) => {
        setPlaceholderNickname(result);
    })

	return (
        <main class="hero min-h-screen text-center">
            <div class="max-w-md">
                <Title>Регистрация</Title>

                <h1 class="text-5xl font-bold py-6">Регистрация</h1>
                <input type="text" placeholder={placeholderNickname()} class="input input-bordered input-accent w-full max-w-xs" />
            </div>
        </main>
    );
}
