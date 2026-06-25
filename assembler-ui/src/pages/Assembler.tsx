import { TypingEffect } from '../components/TypingEffect';
import { Card } from '../components/Card';

export const Assembler = () => {
  return (
    <div className="space-y-6">
      <Card header={<TypingEffect text="SIC/XE ASSEMBLER" speed={50} />} headerColor="bg-retro-green">
        <p className="font-mono text-electric-blue">
          
        </p>
      </Card>
    </div>
  );
};