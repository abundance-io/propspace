import { Link } from "react-router-dom";

export const ButtonLink = (props) => {
  const { text, onClick, variant, url } = props;
  return (
    <Link
      to={url}
      onClick={onClick}
      className={`${variant == "outline" ? "hover:border-mustard border-[1px] border-white" : "bg-white"} hover:bg-mustard rounded-[12px] px-6 py-4 text-sm
        font-semibold text-black transition duration-200 ease-in-out`}
    >
      {text}
    </Link>
  );
};
