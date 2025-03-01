const axios = require("axios");

// Base URL for your API endpoints.
const BASE_URL = "http://127.0.0.1:3001";
const duration = 10; // duration is minutes, calculate in milliseconds done later

// Create a new post by issuing a POST to the "/" endpoint.
// Adjust the payload fields based on your API's expected form.
async function createPost() {
  try {
    const payload = {
      title: "Stress Test Post " + Date.now(),
      text: "This post was created by the stress test."
    };
    const res = await axios.post(`${BASE_URL}/api/post`, payload, {
      headers: { "Content-Type": "application/json" }
    });
    console.log("Created Post:", res.data);
    return res.data; // assume response includes the created post with its ID
  } catch (err) {
    console.error("Error creating post:", err.message);
    return null;
  }
}

// Get the details of a post by its ID using the GET "/{id}" endpoint.
async function getPostById(id) {
  try {
    const res = await axios.get(`${BASE_URL}/api/post/${id}`);
    console.log("Fetched Post by ID:", res.data);
  } catch (err) {
    console.error("Error fetching post by ID:", err.message);
  }
}

// List all posts from the "/" GET endpoint.
async function listPosts() {
  try {
    const res = await axios.get(`${BASE_URL}/api/posts`);
    console.log("Posts List:", res.data);
  } catch (err) {
    console.error("Error listing posts:", err.message);
  }
}

// This function randomly picks an action, simulating organic traffic.
async function stressTestIteration() {
  const action = Math.random();

  if (action < 0.33) {
    // Create a new post and then, after a short delay, fetch it by ID.
    const created = await createPost();
    if (created && created.id) {
      setTimeout(() => {
        getPostById(created.id);
      }, 500);
    }
  } else if (action < 0.66) {
    // Simply list all posts.
    await listPosts();
  } else {
    // Create a post, then list posts, then later fetch the created post.
    const created = await createPost();
    await listPosts();
    if (created && created.id) {
      setTimeout(() => {
        getPostById(created.id);
      }, 500);
    }
  }
}

// Run the stress test for a given number of milliseconds.
async function startStressTest(durationMs) {
  const startTime = Date.now();
  while (Date.now() - startTime < durationMs) {
    await stressTestIteration();
    // Wait a random delay between 200ms and 800ms between iterations.
    const delay = Math.floor(Math.random() * 600) + 200;
    await new Promise((resolve) => setTimeout(resolve, delay));
  }
  console.log("Stress test completed.");
}

const milliseconds = duration * 60 * 1000;

startStressTest(milliseconds);