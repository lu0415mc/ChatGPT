import { Form, Switch, Input, InputNumber, Tooltip } from 'antd';
import { QuestionCircleOutlined } from '@ant-design/icons';

// import SwitchOrigin from '@/components/SwitchOrigin';
import { DISABLE_AUTO_COMPLETE } from '@/utils';

const MainCloseLabel = () => {
  return (
    <span>
      Close Exit{' '}
      <Tooltip title="Click the close button whether to exit directly, the default minimized.">
        <QuestionCircleOutlined style={{ color: '#1677ff' }} />
      </Tooltip>
    </span>
  );
};

export default function MainWindow() {
  return (
    <>
      <Form.Item label={<MainCloseLabel />} name="main_close" valuePropName="checked">
        <Switch />
      </Form.Item>
      <Form.Item label="Default Width" name="main_width">
        <InputNumber />
      </Form.Item>
      <Form.Item label="Default Height" name="main_height">
        <InputNumber />
      </Form.Item>
      {/* <SwitchOrigin name="main" /> */}
      <Form.Item label="User Agent (Main)" name="ua_window">
        <Input.TextArea
          autoSize={{ minRows: 4, maxRows: 4 }}
          {...DISABLE_AUTO_COMPLETE}
          placeholder="Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"
        />
      </Form.Item>
    </>
  );
}
