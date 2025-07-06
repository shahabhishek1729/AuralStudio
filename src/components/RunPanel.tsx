import { SetStateAction, useState } from "react";
import { RDisplay } from "../typing/display";
import { CheckmarkIcon, ErrorIcon, WindowButton } from "./Components";

export default function RunPanel(
  runDisplay: RDisplay,
  setRDisplay: React.Dispatch<SetStateAction<RDisplay>>,
  output: string,
  err: string | null,
) {
  const [showWindowText, setShowWindowText] = useState(false);

  return runDisplay === "FLOAT" ? (
    <div
      className="flex rounded-full"
      style={{
        width: "fit-content",
        background: "linear-gradient(to right, #49A7FE, #2C6498)",
        border: "2px solid #49A7FE",
        padding: "5px",
        cursor: "pointer",
      }}
      onClick={() => setRDisplay("PANEL")}
    >
      <svg
        width="70"
        height="70"
        viewBox="0 0 69 76"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M64.0974 18.7207L67.1945 20.7651C67.8184 21.1769 67.9922 22.017 67.5826 22.6416L62.3912 30.5574C61.9817 31.1819 61.1439 31.3543 60.52 30.9425L57.0858 28.6757C56.4619 28.2639 56.2881 27.4238 56.6977 26.7992C57.6225 25.3891 58.1319 24.5552 56.8415 23.1526C55.7402 21.9554 52.1079 19.573 50.7288 21.6759L49.0871 24.1793C48.6847 24.7927 47.8693 24.97 47.2492 24.5858C47.238 24.5789 47.2268 24.5717 47.2158 24.5644C45.663 23.5395 43.7598 22.619 42.3911 21.3586C41.8284 20.8404 41.7903 19.927 42.3098 19.3291C43.3494 18.1326 44.5882 16.4152 43.7213 15.843C42.3423 14.9327 41.3424 14.236 39.6927 13.4411C36.6965 11.9972 32.2703 11.384 29.2082 11.1244C28.7668 11.087 28.7105 10.3293 29.1444 10.241C29.1512 10.2396 29.1529 10.2394 29.1597 10.2376C29.3746 10.1813 36.4863 8.32738 41.1341 8.56209C44.1792 8.71586 45.9642 8.84769 48.8282 9.90053C52.4972 11.2493 55.6753 13.1614 57.1034 15.5477C58.5316 17.9339 56.8947 19.6301 58.774 20.2058C59.901 20.5511 61.1275 20.285 61.8386 19.3138C62.3626 18.5981 63.3563 18.2315 64.0974 18.7207Z"
          fill="white"
        />
        <rect
          width="5.57999"
          height="20.5532"
          transform="matrix(0.839614 0.543184 -0.540707 0.841211 43.002 20.6377)"
          fill="white"
        />
        <rect
          width="9.17833"
          height="32.4045"
          rx="2"
          transform="matrix(0.835481 0.549519 -0.547037 0.837108 30.834 36.1426)"
          fill="white"
        />
      </svg>
    </div>
  ) : (
    <div className="h-75 w-110 relative">
      <div
        className="h-70 w-100 absolute"
        style={{
          background: "linear-gradient(#FFFFFF1A, #191A1BFF)",
          borderRadius: "42px",
          padding: "2px",
          zIndex: "2",
          backdropFilter: "blur(8px)",
        }}
      >
        <div
          className="h-full w-full"
          style={{
            background: "radial-gradient(circle, #292B2C80, #18191B66)",
            borderRadius: "40px",
          }}
        >
          <div className="flex flex-col w-full absolute top-6 left-6">
            <div className="flex flex-row" style={{ gap: "8px" }}>
              {WindowButton(
                "#FF605C",
                () => setRDisplay("FLOAT"),
                showWindowText,
                () => setShowWindowText(!showWindowText),
              )}
              {WindowButton(
                "#FFBD44",
                () => setRDisplay("FLOAT"),
                showWindowText,
                () => setShowWindowText(!showWindowText),
              )}
              {WindowButton(
                "#00CA4E",
                () => {},
                showWindowText,
                () => setShowWindowText(!showWindowText),
              )}
            </div>

            <div id="runStatus" className="flex flex-row mt-6 items-center">
              <h1
                className="mr-3 font-bold text-5xl"
                style={{ fontFamily: "JetBrains Mono" }}
              >
                {err ? "Failed" : "Success"}
              </h1>
              <div className="mt-2">
                {err ? <ErrorIcon /> : <CheckmarkIcon />}
              </div>
            </div>

            <p
              className="mt-3 text-lg"
              style={{ fontFamily: "JetBrains Mono" }}
            >
              {output}
              <br />
              {err}
            </p>

            <div
              className="rounded-full py-2 text-2xl mt-8"
              style={{
                width: "85%",
                background: "linear-gradient(to right, #49A7FE, #2C6498)",
                textAlign: "center",
                cursor: "pointer",
              }}
              onClick={() =>
                window.dispatchEvent(new KeyboardEvent("keyup", { key: "r" }))
              }
            >
              Run Code
            </div>
          </div>
        </div>
      </div>
      <div
        className="h-20 w-20 top-6 -left-8 absolute rounded-full z-[1]"
        style={{
          background: "radial-gradient(#49A7FEFF, #2C6498FF)",
        }}
      />
      <div
        className="h-50 w-50 -top-9 right-0 absolute rounded-full z-[1]"
        style={{
          background: "radial-gradient(#49A7FEFF, #2C6498FF)",
        }}
      />
      <div
        className="h-10 w-10 bottom-13 -left-2 absolute rounded-full z-[1]"
        style={{
          background: "radial-gradient(#49A7FEFF, #2C6498FF)",
        }}
      />
      <div
        className="h-18 w-18 bottom-3 right-6 absolute rounded-full z-[1]"
        style={{
          background: "radial-gradient(#49A7FEFF, #2C6498FF)",
        }}
      />
    </div>
  );
}
