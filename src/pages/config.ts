import { useEffect, useMemo, useState } from "react";

const config = {
  logsFilterText: "",
  connectionsFilterText: "",
};

export function useConfig(
  type: "logs" | "connections"
): [string, (text: string) => void] {
  if (type === "logs") {
    const [filterText, setFilterText] = useState(config.logsFilterText);
    return [
      filterText,
      function (e: string) {
        config.logsFilterText = e;
        setFilterText(e);
      },
    ];
  } else if (type === "connections") {
    const [filterText, setFilterText] = useState(config.connectionsFilterText);
    return [
      filterText,
      function (e: string) {
        config.connectionsFilterText = e;
        setFilterText(e);
      },
    ];
  } else {
    throw new Error("Invalid type");
  }
}
