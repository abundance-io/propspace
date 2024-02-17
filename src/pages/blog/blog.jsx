import { Link } from "react-router-dom";
import { BlogPosts } from "../../data/blogs";
import grid from "../../assets/images/grid.png";

export const AllBlogs = () => {
  return (
    <section
      className="flex h-screen flex-col items-center space-y-10 pt-[200px]"
      style={{
        backgroundImage: `url(${grid})`,
        backgroundSize: "cover",
        backgroundPosition: "center",
        backgroundRepeat: "no-repeat",
      }}
    >
      <h1 className="text-9xl font-medium">All Posts</h1>
      <ul className="px-48">
        {BlogPosts.map((post) => (
          <li
            key={post.id}
            className="flex h-fit flex-col gap-4 rounded-[40px] border-[1px] border-white bg-black p-10"
          >
            <h1 className="text-mustard text-2xl uppercase">
              <Link to="intro">{post.title}</Link>
            </h1>
            <p>{post.description}</p>
            <p className="text-thin">
              {post.date} - {post.author}
            </p>
          </li>
        ))}
      </ul>
    </section>
  );
};
