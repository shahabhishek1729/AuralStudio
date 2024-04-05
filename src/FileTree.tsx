import React, { useEffect, useState } from "react";
import ArrowRightIcon from "@mui/icons-material/ArrowRight";
import { styled, useTheme } from "@mui/material/styles";
import ArrowDropDownIcon from "@mui/icons-material/ArrowDropDown";
import { TreeView } from "@mui/x-tree-view/TreeView";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import { SvgIconProps } from "@mui/material/SvgIcon";
import {
  TreeItem,
  TreeItemProps,
  treeItemClasses,
} from "@mui/x-tree-view/TreeItem";
import { invoke } from "@tauri-apps/api/tauri";

declare module "react" {
  interface CSSProperties {
    "--tree-view-color"?: string;
    "--tree-view-bg-color"?: string;
  }
}

type StyledTreeItemProps = TreeItemProps & {
  bgColor?: string;
  bgColorForDarkMode?: string;
  color?: string;
  colorForDarkMode?: string;
  labelIcon: React.ElementType<SvgIconProps>;
  labelInfo?: string;
  labelText: string;
};

const StyledTreeItemRoot = styled(TreeItem)(({ theme }) => ({
  color: theme.palette.text.secondary,
  [`& .${treeItemClasses.content}`]: {
    color: theme.palette.text.secondary,
    borderTopRightRadius: theme.spacing(2),
    borderBottomRightRadius: theme.spacing(2),
    paddingRight: theme.spacing(1),
    fontWeight: theme.typography.fontWeightMedium,
    "&.Mui-expanded": {
      fontWeight: theme.typography.fontWeightRegular,
    },
    "&:hover": {
      backgroundColor: theme.palette.action.hover,
    },
    "&.Mui-focused, &.Mui-selected, &.Mui-selected.Mui-focused": {
      backgroundColor: `var(--tree-view-bg-color, ${theme.palette.action.selected})`,
      color: "var(--tree-view-color)",
    },
    [`& .${treeItemClasses.label}`]: {
      fontWeight: "inherit",
      color: "inherit",
    },
  },
  [`& .${treeItemClasses.group}`]: {
    marginLeft: 0,
    [`& .${treeItemClasses.content}`]: {
      paddingLeft: theme.spacing(2),
    },
  },
})) as unknown as typeof TreeItem;

const StyledTreeItem = React.forwardRef(function StyledTreeItem(
  props: StyledTreeItemProps,
  ref: React.Ref<HTMLLIElement>
) {
  const theme = useTheme();
  const {
    bgColor,
    color,
    labelIcon: LabelIcon,
    labelInfo,
    labelText,
    colorForDarkMode,
    bgColorForDarkMode,
    ...other
  } = props;

  const styleProps = {
    "--tree-view-color":
      theme.palette.mode !== "dark" ? color : colorForDarkMode,
    "--tree-view-bg-color":
      theme.palette.mode !== "dark" ? bgColor : bgColorForDarkMode,
  };

  return (
    <StyledTreeItemRoot
      label={
        <Box
          sx={{
            display: "flex",
            alignItems: "center",
            p: 0.5,
            pr: 0,
          }}
        >
          <Box component={LabelIcon} color="white" sx={{ mr: 1 }} />
          <Typography
            variant="body2"
            sx={{
              fontWeight: "inherit",
              flexGrow: 1,
              color: "white",
              fontFamily: "Inter",
            }}
          >
            {labelText}
          </Typography>
          <Typography variant="caption" color="inherit">
            {labelInfo}
          </Typography>
        </Box>
      }
      style={styleProps}
      {...other}
      ref={ref}
    />
  );
});

function BuildTreeItem(obj, id: number) {
  return (
    <StyledTreeItem key={id} nodeId={id.toString()} labelText={obj.name}>
      {(obj.children || []).map((c) => {
        id++;
        return BuildTreeItem(c, id);
      })}
    </StyledTreeItem>
  );
}

export function FileTree() {
  let [hier, setHier] = useState({ name: ".", children: [] });
  useEffect(() => {
    invoke("get_file_hierarchy", { rootPath: "." }).then((obj) => setHier(obj));
  }, []);

  return (
    <TreeView
      aria-label="gmail"
      defaultExpanded={["3"]}
      defaultCollapseIcon={
        <ArrowDropDownIcon
          style={{ color: "#98A2B3", scale: "1.2", marginRight: "-20" }}
        />
      }
      defaultExpandIcon={
        <ArrowRightIcon
          style={{ color: "#98A2B3", scale: "1.2", marginRight: "-20" }}
        />
      }
      defaultEndIcon={<div style={{ width: 24 }} />}
      sx={{ height: 264, flexGrow: 1, maxWidth: 400, overflowY: "auto" }}
      style={{ marginTop: "20px", height: "100vh" }}
    >
      {/*<StyledTreeItem nodeId="1" labelText="lib" >
          <StyledTreeItem />
        </StyledTreeItem>
        <StyledTreeItem nodeId="2" labelText="bin" >
          <StyledTreeItem />
        </StyledTreeItem>
        <StyledTreeItem nodeId="3" labelText="src">
          <StyledTreeItem
            nodeId="5"
            labelText="arithmetic_calculator" >
            <StyledTreeItem
              nodeId="6"
              labelText="add_and_divide" >
              <StyledTreeItem
                nodeId="7"
                labelText="addition.py" />
              <StyledTreeItem
                nodeId="8"
                labelText="addition.rattle" />
            </StyledTreeItem>
            <StyledTreeItem
              nodeId="9"
              labelText="multiply_and_subtract" />
          </StyledTreeItem>

          <StyledTreeItem
            nodeId="10"
            labelText="algebraic_calculator" >
          </StyledTreeItem>
        </StyledTreeItem>
        <StyledTreeItem nodeId="11" labelText="include" >
          <StyledTreeItem />
        </StyledTreeItem> */}
      {BuildTreeItem(hier, 0)}
    </TreeView>
  );
}
