import { Stepper } from "@mantine/core";
import { useEffect, useState } from "react";
import { debug, error, warn } from "../logger";
import { invoke } from "@tauri-apps/api";
import FormInput from "./FormInput";

export default function LoginFlow() {

    const [activeStep, setActiveStep] = useState(0);
    const [instanceConfig, setInstanceConfig] = useState(null);
    const [username, setUsername] = useState(null);

    const [authMechanisms, setAuthMechanisms] = useState([]);

    useEffect(() => {
        invoke('current_config').then((cfg) => {
            debug("setting current config to: {}", cfg);
            setInstanceConfig(cfg);
            setActiveStep(1);
        }).catch((err) => {
            error("error while getting config: {}", err);
        });
    }, []);

    useEffect(() => {

        if (!username) {
            warn("username was falsy");
            return;
        }

        invoke('connect', {user: username}).then((mechs) => {
            debug("available mechanisms: {}", mechs);
            setAuthMechanisms(mechs);
        }).catch((err) => {
            error("error using connect with username {}. {}", username, err);
        })

    }, [username]);

    const handleUsername = (username) => {
        setUsername(username);
    };

    return (
        <Stepper active={activeStep} orientation="horizontal">
            <Stepper.Step label="Instance Selection">
                todo instance selection
            </Stepper.Step>
            <Stepper.Step label="User Identification">
                <FormInput legend={'User Identification'} label={'Username'} placeholder={'Your Username'} isPassword={false} onSubmit={handleUsername}/>
            </Stepper.Step>
            <Stepper.Step label="Authentication Mechanism">
                oof2
            </Stepper.Step>
            <Stepper.Step label="Authenticate">
                oof3
            </Stepper.Step>
        </Stepper>
    );

};