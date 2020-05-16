import React, { useState } from 'react';
import styled from 'styled-components';
import { ChatChannel } from "../../models/NodeKakaoPureObject";
import ChatListItem from './ChatListItem';
import ProfileDefault from '../../assets/images/profile_default.svg'
import color from '../../assets/javascripts/color';

const Wrapper = styled.div`
width: 309px;
flex: 1;
display: flex;
flex-direction: column;
background: ${color.GREY_900};
overflow-y: scroll;
::-webkit-scrollbar {
  width: 0px;
  background: transparent;
}
`;

interface ChatListProps {
  channelList: ChatChannel[]
  onChange?: (index: number) => any;
}

const ChatList: React.FC<ChatListProps> = ({channelList, onChange}) => {
  const [selectedIndex, setSelectedIndex] = useState(0);
  return (
    <Wrapper>
      {channelList.map((channel, index) => <ChatListItem key={channel.id.low} lastChat={channel.lastChat ? channel.lastChat.text : ''} profileImageSrc={channel.channelInfo.roomImageURL || ProfileDefault} username={channel.channelInfo.name} selected={selectedIndex === index} onClick={() => {
        setSelectedIndex(index);
        onChange && onChange(index);
      }}/>)}
    </Wrapper>
  );
};

export default ChatList;
