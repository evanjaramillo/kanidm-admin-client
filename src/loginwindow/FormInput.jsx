import PropTypes from 'prop-types';
import { Button, Fieldset, Group, TextInput } from "@mantine/core";
import { useState } from "react";

export default function FormInput({label, placeholder, legend, isPassword, onSubmit}) {

    const [content, setContent] = useState("");

    return (
        <Fieldset legend={legend}>
            <TextInput label={label} placeholder={placeholder}
                value={content}
                type={isPassword ? 'password' : 'text'}
                onChange={(e) => setContent(e.currentTarget.value)} />
            <Group justify="flex-end" mt={"md"}>
                <Button onClick={() => onSubmit(content)}>Submit</Button>
            </Group>
        </Fieldset>
    );
}

FormInput.propTypes = {
    label: PropTypes.string,
    placeholder: PropTypes.string,
    legend: PropTypes.string,
    isPassword: PropTypes.bool,
    onSubmit: PropTypes.func
};