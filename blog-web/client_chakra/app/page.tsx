import React from 'react';

const Home = async () => {
  const apiUrl = process.env.NEXT_PUBLIC_API_URL;
  const envName = process.env.ENV_NAME;

  console.log(envName);

  return (
      <main className="flex min-h-screen flex-col items-center justify-between p-24">
        <p>{apiUrl}</p>
        <p>{envName}</p>
      </main>
  );
}

export default Home;

