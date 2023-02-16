const Content = () => {
  return (
    <div className="content_container">
      <video width="750" height="500" controls>
        <source
          src="https://www.youtube.com/watch?v=dQw4w9WgXcQ"
          type="video/mp4"
        />
      </video>
      <div>Description</div>
    </div>
  );
};

export default Content;
