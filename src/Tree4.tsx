import { ArcherContainer, ArcherElement } from "react-archer";
import gear from "./assets/action_cpu.png";
import question from "./assets/conditional_q4.png";
import cube from "./assets/value_box.png";
import book from "./assets/vuesax/bold/book.png";
import rattle_icon from "./assets/rattle_icon.png";

const TOKEN_MAP = {
  file: ["#FFFFFF", rattle_icon],
  function: ["#FE4949", gear],
  variable: ["#49A7FE", cube],
  conditional: ["#06975A", question],
  library: ["#B140B4", book],
};

const rootStyle = { display: "flex", justifyContent: "center" };
const rowStyle = {
  margin: "200px 0",
  display: "flex",
  justifyContent: "space-between",
};
const boxStyle = { padding: "10px", border: "1px solid black" };

function FileToken({ fname }) {
  return (
    <div
      style={{
        display: "flex",
        background: "white",
        height: "40px",
        width: "fit-content",
        borderRadius: "10px",
        flexDirection: "row",
        justifyContent: "center", //Centered vertically
        alignItems: "center", //Centered horizontally
        paddingLeft: "60px",
        paddingRight: "10px",
      }}
    >
      <img
        src={rattle_icon}
        height="32px"
        style={{
          marginLeft: "-48px",
          marginRight: "4px",
        }}
      />

      <div
        style={{ height: "40px", width: "1px", backgroundColor: "#000000" }}
      ></div>

      <p
        style={{
          // width: "90px",
          fontFamily: "JetBrains Mono",
          textAlign: "start",
          marginLeft: "7px",
          color: "black",
        }}
      >
        {fname}
      </p>
    </div>
  );
}

export function Token({ token_type, metadata }) {
  return (
    <div
      style={{
        background: TOKEN_MAP[token_type][0],
        height: "40px",
        width: "fit-content",
        borderRadius: "10px",
        display: "flex",
        flexDirection: "row",
        justifyContent: "center", //Centered vertically
        alignItems: "center", //Centered horizontally
        paddingLeft: "15px",
        paddingRight: "15px",
      }}
    >
      <img
        src={TOKEN_MAP[token_type][1]}
        height="32px"
        style={{
          marginLeft: "-8px",
          marginRight: "4px",
        }}
      />

      <div
        style={{ height: "40px", width: "1px", backgroundColor: "#FFFFFF" }}
      ></div>

      <p
        style={{
          fontFamily: "JetBrains Mono",
          textAlign: "start",
          marginLeft: "7px",
        }}
      >
        {metadata["name"]}
      </p>
    </div>
  );
}

const childNodes = [
  {
    type: "library",
    name: "numpy",
  },
  {
    type: "library",
    name: "pandas",
  },
  {
    type: "library",
    name: "matplotlib",
  },
];

function TestTree() {
  return (
    <div style={{ height: "500px", margin: "50px" }}>
      <ArcherContainer strokeColor="white">
        <div style={rootStyle}>
          <ArcherElement
            id="root"
            relations={
              // [
              //   {
              // targetId: "element4",
              // targetAnchor: "top",
              // sourceAnchor: "bottom",
              //   },
              // ]
              childNodes.map((d, idx) => {
                return {
                  targetId: `element${idx}`,
                  targetAnchor: "top",
                  sourceAnchor: "bottom",
                };
              })
            }
          >
            <div
              style={{
                display: "flex",
                background: "white",
                height: "40px",
                width: "fit-content",
                borderRadius: "10px",
                flexDirection: "row",
                justifyContent: "center", //Centered vertically
                alignItems: "center", //Centered horizontally
                paddingLeft: "60px",
                paddingRight: "10px",
              }}
            >
              <img
                src={rattle_icon}
                height="32px"
                style={{
                  marginLeft: "-48px",
                  marginRight: "4px",
                }}
              />

              <div
                style={{
                  height: "40px",
                  width: "1px",
                  backgroundColor: "#000000",
                }}
              ></div>

              <p
                style={{
                  // width: "90px",
                  fontFamily: "JetBrains Mono",
                  textAlign: "start",
                  marginLeft: "7px",
                  color: "black",
                }}
              >
                helloworld.rattle
              </p>
            </div>
          </ArcherElement>
        </div>

        <div style={rowStyle}>
          {/* <ArcherElement */}
          {/* id="element2" */}
          {/* relations={[ */}
          {/* {
                targetId: "element3",
                targetAnchor: "left",
                sourceAnchor: "right",
                style: { strokeColor: "blue", strokeWidth: 1 },
                label: <div style={{ marginTop: "-20px" }}>Arrow 2</div>,
              },
            ]}
          > */}
          {/* <Token token_type={"library"} metadata={{ name: "numpy" }} /> */}
          {/* <Token token_type={"library"} metadata={{ name: "pandas" }} />
            <Token token_type={"variable"} metadata={{ name: "global" }} />
            <Token token_type={"function"} metadata={{ name: "execute1()" }} />
            <Token token_type={"function"} metadata={{ name: "execute2()" }} /> */}
          {/* </ArcherElement> */}

          {/* <ArcherElement id="element3"> */}
          {/* <Token token_type={"library"} metadata={{ name: "numpy" }} /> */}
          {/* </ArcherElement> */}

          {childNodes.map((d, idx) => {
            return (
              <ArcherElement id={`element${idx}`} relations={[]}>
                <div
                  style={{
                    background: TOKEN_MAP[d.type][0],
                    height: "40px",
                    width: "fit-content",
                    borderRadius: "10px",
                    display: "flex",
                    flexDirection: "row",
                    justifyContent: "center", //Centered vertically
                    alignItems: "center", //Centered horizontally
                    paddingLeft: "15px",
                    paddingRight: "15px",
                  }}
                >
                  <img
                    src={TOKEN_MAP[d.type][1]}
                    height="32px"
                    style={{
                      marginLeft: "-8px",
                      marginRight: "4px",
                    }}
                  />

                  <div
                    style={{
                      height: "40px",
                      width: "1px",
                      backgroundColor: "#FFFFFF",
                    }}
                  ></div>

                  <p
                    style={{
                      fontFamily: "JetBrains Mono",
                      textAlign: "start",
                      marginLeft: "7px",
                    }}
                  >
                    {d.name}
                  </p>
                </div>
              </ArcherElement>
            );
          })}
        </div>
      </ArcherContainer>
    </div>
  );
}

export default TestTree;
