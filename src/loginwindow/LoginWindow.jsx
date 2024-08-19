import { Image, Stack } from "@mantine/core";
import LoginFlow from "./LoginFlow";

export default function LoginWindow() {

    return (
        <div style={{padding: '10px'}}>
            <Stack gap="md" justify="center" pt={5}>
                <Image src="/logo-small.png" fit="contain" h={100} w={200} style={{ display: 'block', margin: 'auto' }} />
                <LoginFlow />
            </Stack>
        </div>
    );
}