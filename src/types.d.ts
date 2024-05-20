declare namespace I {
  export type AppConf = {
    theme: 'light' | 'dark' | 'system';
    stay_on_top: boolean;
    ask_mode: boolean;
  }

  export interface SVG extends React.SVGProps<SVGSVGElement> {
    children?: React.ReactNode;
    size?: number;
    title?: string;
    action?: boolean;
    onClick?: (e: React.MouseEvent) => void;
  }
}