import React from "react";

interface VNodeRendererProps {
  nodes: {
    type: string;
    props: React.ImgHTMLAttributes<HTMLImageElement>;
  };
}

const VNodeRenderer: React.FC<VNodeRendererProps> = ({ nodes }) => {
  if (!nodes) return null;

  switch (nodes.type) {
    case "img":
      return <img {...nodes.props} />;
    default:
      return null;
  }
};

export default VNodeRenderer;
