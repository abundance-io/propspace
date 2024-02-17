import { BlogPosts } from "../../data/blogs";
import Markdown from "react-markdown";

export const BlogEntry = () => {
  return (
    <section className=" p-20">
      {BlogPosts.map((post) => (
        <div key={post.id} className="flex">
          <div className="flex flex-col gap-8">
            <h1 className="text-9xl">{post.title}</h1>
            <p className="text-mustard">
              {post.date} - {post.author}
            </p>
          </div>
          <p className="text-xl">
            <Markdown>{post.content}</Markdown>
          </p>
        </div>
      ))}
    </section>
  );
};
