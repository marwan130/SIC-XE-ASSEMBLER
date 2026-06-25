import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { Button } from '../components/Button';
import { Card } from '../components/Card';
import { TypingEffect } from '../components/TypingEffect';
import { authService } from '../lib/auth';

export const Login = () => {
  const navigate = useNavigate();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    try {
      await authService.login({ email, password });
      navigate('/');
    } catch (err: any) {
      setError(err.response?.data?.message || 'Login failed');
    }
  };

  return (
    <div className="flex items-center justify-center min-h-[calc(100vh-200px)]">
      <Card header={<TypingEffect text="LOGIN" speed={100} />} headerColor="bg-retro-green">
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-bold mb-2 font-mono text-electric-blue">
              EMAIL
            </label>
            <input
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="w-full p-2 bg-dark-bg border-2 border-black font-mono focus:outline-none focus:border-retro-green"
              required
            />
          </div>
          <div>
            <label className="block text-sm font-bold mb-2 font-mono text-electric-blue">
              PASSWORD
            </label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full p-2 bg-dark-bg border-2 border-black font-mono focus:outline-none focus:border-retro-green"
              required
            />
          </div>
          {error && <p className="text-red-500 font-mono text-sm">{error}</p>}
          <Button type="submit" variant="primary" className="w-full" magnetic>
            LOGIN
          </Button>
          <p className="text-center font-mono text-sm">
            Don't have an account?{' '}
            <Link to="/register" className="text-retro-green hover:animate-flash">
              REGISTER
            </Link>
          </p>
        </form>
      </Card>
    </div>
  );
};