import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { Button } from '../components/Button';
import { Card } from '../components/Card';
import { TypingEffect } from '../components/TypingEffect';
import { authService } from '../lib/auth';

export const Register = () => {
  const navigate = useNavigate();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [error, setError] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    if (password !== confirmPassword) {
      setError('Passwords do not match');
      return;
    }
    try {
      await authService.register({ email, password });
      navigate('/');
    } catch (err: any) {
      setError(err.response?.data?.message || 'Registration failed');
    }
  };

  return (
    <div className="flex items-center justify-center min-h-[calc(100vh-200px)]">
      <Card header={<TypingEffect text="REGISTER" speed={100} />} headerColor="bg-cyber-yellow">
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
          <div>
            <label className="block text-sm font-bold mb-2 font-mono text-electric-blue">
              CONFIRM PASSWORD
            </label>
            <input
              type="password"
              value={confirmPassword}
              onChange={(e) => setConfirmPassword(e.target.value)}
              className="w-full p-2 bg-dark-bg border-2 border-black font-mono focus:outline-none focus:border-retro-green"
              required
            />
          </div>
          {error && <p className="text-red-500 font-mono text-sm">{error}</p>}
          <Button type="submit" variant="primary" className="w-full" magnetic>
            REGISTER
          </Button>
          <p className="text-center font-mono text-sm">
            Already have an account?{' '}
            <Link to="/login" className="text-retro-green hover:animate-flash">
              LOGIN
            </Link>
          </p>
        </form>
      </Card>
    </div>
  );
};