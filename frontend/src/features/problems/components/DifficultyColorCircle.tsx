type Props = {
  difficulty?: number;
  fillPercent?: number;
  size?: string;
};

export const DifficultyColorCircle = ({
  difficulty,
  fillPercent = 1,
  size = "12px",
}: Props) => {
  const color = difficulty ? "blue" : "black";

  const circleStyle: React.CSSProperties = {
    width: size,
    height: size,
    borderRadius: "50%",
    border: `1px solid ${color}`,
    background: `linear-gradient(to top, ${color} ${fillPercent * 100}%, transparent ${fillPercent * 100}%)`,
    display: "inline-block",
  };

  return <span aria-label="colored circle" style={circleStyle} />;
};
