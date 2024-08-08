import React, {useState} from "react";
import CloseIcon from '@mui/icons-material/Close';
import tab_3x from "./assets/active_tab_1x.png";
import notab_3x from "./assets/notab_1x.png";

function TabsComponent() {
  const [selectedIdx, setSelectedIdx] = useState(1);
  
  return (
  <div style={{backgroundColor: "#111315", display: "flex", flexDirection: "row", borderBottom: "0.3px solid #636363"}}>
  {["addition.rattle", "linalg.rattle"].map((item, index) => (
    <div key={index} style={{display: "flex", flexDirection: "row", backgroundImage: 'url("./assets/tab_3x.png")', marginRight: "0px", alignItems: "center"}}>
      <div style={{display: "block", position: "relative"}}>
      <div>
        {selectedIdx === index ? <img src={tab_3x} /> : <img src={notab_3x} /> }
      </div>

      <div style={{position: "absolute", right: "10%", left: "10%", bottom: "0%"}}>
        <p>{item}</p>
      </div>

      <div style={{position: "absolute", right: "10%", left: "87%", bottom: "17%"}}>
        <CloseIcon style={{backgroundColor: "transparent", width: "16px", paddingRight: "10px"}}/>
      </div>
      </div>
      {/*<button style={{paddingRight: "30px", backgroundColor: "transparent"}}>{item}</button> */}
    </div>
  ))}
  </div>);
}

export default TabsComponent;
