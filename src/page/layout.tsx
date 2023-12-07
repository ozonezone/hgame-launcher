import { Input } from "@nextui-org/react";
import { Button } from "@nextui-org/react";
import { useEffect, useState } from "preact/hooks";
import { Config, configRead, configWrite, scanStart } from "../bindings";

export default function Layout() {
  const [config, setConfig] = useState<Config | null>(null);
  useEffect(() => {
    (async () => {
      setConfig(await configRead());
    })();
  }, []);

  const [value, setValue] = useState<string>("");

  return (
    <div className="flex w-wull p-2 flex-col">
      <div className="flex">
        <Input
          type="text"
          label="Text"
          placeholder=""
          value={value}
          onChange={(e) => setValue(e.target.value)}
        />
        <Button
          color="primary"
          onClick={async () => {
            if (config) {
              const c = {
                ...config,
                scan_dir: [value],
              };
              await configWrite(c);
            }
          }}
        >
          Set
        </Button>
      </div>

      <Button
        color="primary"
        onClick={async () => {
          scanStart();
        }}
      >
        Start
      </Button>
      <div>
        <pre>
      {JSON.stringify(config, null, 2)}
        </pre>
      </div>
    </div>
  );
}
