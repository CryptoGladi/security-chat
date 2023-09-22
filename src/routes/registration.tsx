import { Title } from "solid-start";
import { faker } from '@faker-js/faker'; // WARN VERY BIG PACKAGE

function get_example_nickame() {
    return faker.person.firstName().toLowerCase() + faker.number.int({min: 1000, max: 9999})
}

export default function Index() {
    let example_nickname = get_example_nickame();

	return (
        <main class="hero min-h-screen text-center">
            <div class="max-w-md">
                <Title>Регистрация</Title>

                <h1 class="text-5xl font-bold py-6">Регистрация</h1>
                <input type="text" placeholder={example_nickname} class="input input-bordered input-accent w-full max-w-xs" />
            </div>
        </main>
    );
}
