import { useEffect, useState, useRef } from "react";

interface SlidingBorderProps {
  selectedAddr: string;
  pieceIx: number[] | null;
}

export function SlidingBorder({ selectedAddr, pieceIx }: SlidingBorderProps) {
  const [borderStyle, setBorderStyle] = useState({
    position: "absolute" as const,
    top: 0,
    left: 0,
    width: 0,
    height: 0,
    border: "2px solid #f7dc28",
    borderRadius: "8px",
    pointerEvents: "none" as const,
    zIndex: 1000,
    transition: "all 0.3s cubic-bezier(0.4, 0, 0.2, 1)",
    opacity: 0,
    boxShadow: "0 0 20px rgba(247, 220, 40, 0.3)",
    animation: "pulse 2s ease-in-out infinite",
  });

  const borderRef = useRef<HTMLDivElement>(null);
  const animationFrameRef = useRef<number>();

  // Add CSS animation for pulsing effect
  useEffect(() => {
    const style = document.createElement("style");
    style.textContent = `
      @keyframes pulse {
        0%, 100% {
          box-shadow: 0 0 20px rgba(247, 220, 40, 0.3);
        }
        50% {
          box-shadow: 0 0 30px rgba(247, 220, 40, 0.6);
        }
      }
    `;
    document.head.appendChild(style);

    return () => {
      document.head.removeChild(style);
    };
  }, []);

  const updateBorderPosition = () => {
    if (!selectedAddr) {
      setBorderStyle((prev) => ({ ...prev, opacity: 0 }));
      return;
    }

    // Try to find the element by various ID patterns
    let targetElement: HTMLElement | null = null;
    let foundPattern = "";

    // Pattern 1: Direct selectedAddr match
    targetElement = document.getElementById(selectedAddr);
    if (targetElement) foundPattern = "direct";

    // Pattern 2: selectedAddr with pieceIx (for input elements)
    if (!targetElement && pieceIx) {
      const pieceIxStr = pieceIx.join(",");
      const id = `${selectedAddr},${pieceIxStr}`;
      targetElement = document.getElementById(id);
      if (targetElement) foundPattern = "with pieceIx string";
    }

    // Pattern 3: selectedAddr with pieceIx array (alternative format)
    if (!targetElement && pieceIx) {
      const id = `${selectedAddr},${pieceIx}`;
      targetElement = document.getElementById(id);
      if (targetElement) foundPattern = "with pieceIx array";
    }

    // Pattern 4: selectedAddr with index (for RenderIdent)
    if (!targetElement) {
      const id = `${selectedAddr},0`;
      targetElement = document.getElementById(id);
      if (targetElement) foundPattern = "with index 0";
    }

    // Pattern 5: selectedAddr with .0 suffix (for nodes)
    if (!targetElement) {
      const id = `${selectedAddr}.0`;
      targetElement = document.getElementById(id);
      if (targetElement) foundPattern = "with .0 suffix";
    }

    // Pattern 6: selectedAddr without .0 suffix (for parent nodes)
    if (!targetElement && selectedAddr.includes(".")) {
      const id = selectedAddr.slice(0, selectedAddr.length - 2);
      targetElement = document.getElementById(id);
      if (targetElement) foundPattern = "without .0 suffix";
    }

    // Pattern 7: selectedAddr with selected_ prefix (existing system)
    if (!targetElement) {
      const id = `selected_${selectedAddr}`;
      targetElement = document.getElementById(id);
      if (targetElement) foundPattern = "with selected_ prefix";
    }

    if (targetElement) {
      const rect = targetElement.getBoundingClientRect();

      // Check if the element is actually visible
      if (rect.width === 0 || rect.height === 0) {
        setBorderStyle((prev) => ({ ...prev, opacity: 0 }));
        return;
      }

      // Get scroll positions from all possible sources
      const scrollX =
        window.scrollX ||
        document.documentElement.scrollLeft ||
        document.body.scrollLeft ||
        0;
      const scrollY =
        window.scrollY ||
        document.documentElement.scrollTop ||
        document.body.scrollTop ||
        0;

      // Add some padding around the element
      const padding = 4;

      // Calculate position relative to the viewport
      const top = rect.top + scrollY - padding;
      const left = rect.left + scrollX - padding;
      const width = rect.width + padding * 2;
      const height = rect.height + padding * 2;

      setBorderStyle((prev) => ({
        ...prev,
        top,
        left,
        width,
        height,
        opacity: 1,
      }));

      // Debug logging (can be removed in production)
      if (process.env.NODE_ENV === "development") {
        console.log(
          `SlidingBorder: Found element with pattern "${foundPattern}" for selectedAddr "${selectedAddr}" at (${left}, ${top}) size (${width}, ${height})`,
        );
      }
    } else {
      setBorderStyle((prev) => ({ ...prev, opacity: 0 }));

      // Debug logging (can be removed in production)
      if (process.env.NODE_ENV === "development") {
        console.log(
          `SlidingBorder: No element found for selectedAddr "${selectedAddr}" with pieceIx`,
          pieceIx,
        );
      }
    }
  };

  useEffect(() => {
    // Update position immediately when selectedAddr changes
    updateBorderPosition();

    // Set up a more frequent update for smooth animations
    const updatePosition = () => {
      updateBorderPosition();
      animationFrameRef.current = requestAnimationFrame(updatePosition);
    };

    // Start the animation loop
    animationFrameRef.current = requestAnimationFrame(updatePosition);

    // Clean up on unmount
    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [selectedAddr, pieceIx]);

  useEffect(() => {
    // Handle window resize and scroll events
    const handleResize = () => {
      updateBorderPosition();
    };

    const handleScroll = () => {
      updateBorderPosition();
    };

    window.addEventListener("resize", handleResize);
    window.addEventListener("scroll", handleScroll, true);

    return () => {
      window.removeEventListener("resize", handleResize);
      window.removeEventListener("scroll", handleScroll, true);
    };
  }, []);

  return <div ref={borderRef} style={borderStyle} />;
}
