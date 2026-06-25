import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { CustomCursor } from './components/CustomCursor';
import { Layout } from './components/Layout';
import { Login } from './pages/Login';
import { Register } from './pages/Register';
import { Assembler } from './pages/Assembler';

function App() {
  return (
    <Router>
      <CustomCursor />
      <Layout>
        <Routes>
          <Route path="/login" element={<Login />} />
          <Route path="/register" element={<Register />} />
          <Route path="/" element={<Assembler />} />
        </Routes>
      </Layout>
    </Router>
  );
}

export default App;