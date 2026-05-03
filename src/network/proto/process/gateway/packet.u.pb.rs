const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.32.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__ToServerMessage_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct ToServerMessage {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ToServerMessage>
}

impl ::protobuf::Message for ToServerMessage {}

impl ::std::default::Default for ToServerMessage {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for ToServerMessage {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for ToServerMessage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for ToServerMessage {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `ToServerMessage` is `Sync` because it does not implement interior mutability.
//    Neither does `ToServerMessageMut`.
unsafe impl Sync for ToServerMessage {}

// SAFETY:
// - `ToServerMessage` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ToServerMessage {}

impl ::protobuf::Proxied for ToServerMessage {
  type View<'msg> = ToServerMessageView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ToServerMessage {}

impl ::protobuf::MutProxied for ToServerMessage {
  type Mut<'msg> = ToServerMessageMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ToServerMessageView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ToServerMessage>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ToServerMessageView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ToServerMessageView<'msg> {
  type Message = ToServerMessage;
}

impl ::std::fmt::Debug for ToServerMessageView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for ToServerMessageView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for ToServerMessageView<'_> {
  fn default() -> ToServerMessageView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    ToServerMessageView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> ToServerMessageView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ToServerMessage>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> ToServerMessage {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // login_request: optional message Proto.PayloadLoginRequest
  pub fn has_login_request(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn login_request_opt(self) -> ::protobuf::Optional<super::PayloadLoginRequestView<'msg>> {
        ::protobuf::Optional::new(self.login_request(), self.has_login_request())
  }
  pub fn login_request(self) -> super::PayloadLoginRequestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLoginRequestView::new(::protobuf::__internal::Private, inner)
  }

  // logout_request: optional message Proto.PayloadLogoutRequest
  pub fn has_logout_request(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn logout_request_opt(self) -> ::protobuf::Optional<super::PayloadLogoutRequestView<'msg>> {
        ::protobuf::Optional::new(self.logout_request(), self.has_logout_request())
  }
  pub fn logout_request(self) -> super::PayloadLogoutRequestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLogoutRequestView::new(::protobuf::__internal::Private, inner)
  }

  // signup_request: optional message Proto.PayloadSignupRequest
  pub fn has_signup_request(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn signup_request_opt(self) -> ::protobuf::Optional<super::PayloadSignupRequestView<'msg>> {
        ::protobuf::Optional::new(self.signup_request(), self.has_signup_request())
  }
  pub fn signup_request(self) -> super::PayloadSignupRequestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadSignupRequestView::new(::protobuf::__internal::Private, inner)
  }

  // start_game: optional message Proto.PayloadLobbyStartGameRequest
  pub fn has_start_game(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn start_game_opt(self) -> ::protobuf::Optional<super::PayloadLobbyStartGameRequestView<'msg>> {
        ::protobuf::Optional::new(self.start_game(), self.has_start_game())
  }
  pub fn start_game(self) -> super::PayloadLobbyStartGameRequestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLobbyStartGameRequestView::new(::protobuf::__internal::Private, inner)
  }

  // end_game: optional message Proto.PayloadLobbyEndGameRequest
  pub fn has_end_game(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn end_game_opt(self) -> ::protobuf::Optional<super::PayloadLobbyEndGameRequestView<'msg>> {
        ::protobuf::Optional::new(self.end_game(), self.has_end_game())
  }
  pub fn end_game(self) -> super::PayloadLobbyEndGameRequestView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLobbyEndGameRequestView::new(::protobuf::__internal::Private, inner)
  }

  // player_zone_enter_complete: optional message Proto.PayloadPlayerZoneEnterComplete
  pub fn has_player_zone_enter_complete(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn player_zone_enter_complete_opt(self) -> ::protobuf::Optional<super::PayloadPlayerZoneEnterCompleteView<'msg>> {
        ::protobuf::Optional::new(self.player_zone_enter_complete(), self.has_player_zone_enter_complete())
  }
  pub fn player_zone_enter_complete(self) -> super::PayloadPlayerZoneEnterCompleteView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadPlayerZoneEnterCompleteView::new(::protobuf::__internal::Private, inner)
  }

  // transform_sync: optional message Proto.PayloadTransformSync
  pub fn has_transform_sync(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn transform_sync_opt(self) -> ::protobuf::Optional<super::PayloadTransformSyncView<'msg>> {
        ::protobuf::Optional::new(self.transform_sync(), self.has_transform_sync())
  }
  pub fn transform_sync(self) -> super::PayloadTransformSyncView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTransformSyncView::new(::protobuf::__internal::Private, inner)
  }

  // play_action: optional message Proto.PayloadPlayAction
  pub fn has_play_action(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn play_action_opt(self) -> ::protobuf::Optional<super::PayloadPlayActionView<'msg>> {
        ::protobuf::Optional::new(self.play_action(), self.has_play_action())
  }
  pub fn play_action(self) -> super::PayloadPlayActionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadPlayActionView::new(::protobuf::__internal::Private, inner)
  }

  // text_message: optional message Proto.PayloadTextMessage
  pub fn has_text_message(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn text_message_opt(self) -> ::protobuf::Optional<super::PayloadTextMessageView<'msg>> {
        ::protobuf::Optional::new(self.text_message(), self.has_text_message())
  }
  pub fn text_message(self) -> super::PayloadTextMessageView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTextMessageView::new(::protobuf::__internal::Private, inner)
  }

  pub fn message(self) -> super::to_server_message::MessageOneof<'msg> {
    match self.message_case() {
      super::to_server_message::MessageCase::LoginRequest =>
          super::to_server_message::MessageOneof::LoginRequest(self.login_request()),
      super::to_server_message::MessageCase::LogoutRequest =>
          super::to_server_message::MessageOneof::LogoutRequest(self.logout_request()),
      super::to_server_message::MessageCase::SignupRequest =>
          super::to_server_message::MessageOneof::SignupRequest(self.signup_request()),
      super::to_server_message::MessageCase::StartGame =>
          super::to_server_message::MessageOneof::StartGame(self.start_game()),
      super::to_server_message::MessageCase::EndGame =>
          super::to_server_message::MessageOneof::EndGame(self.end_game()),
      super::to_server_message::MessageCase::PlayerZoneEnterComplete =>
          super::to_server_message::MessageOneof::PlayerZoneEnterComplete(self.player_zone_enter_complete()),
      super::to_server_message::MessageCase::TransformSync =>
          super::to_server_message::MessageOneof::TransformSync(self.transform_sync()),
      super::to_server_message::MessageCase::PlayAction =>
          super::to_server_message::MessageOneof::PlayAction(self.play_action()),
      super::to_server_message::MessageCase::TextMessage =>
          super::to_server_message::MessageOneof::TextMessage(self.text_message()),
      _ => super::to_server_message::MessageOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn message_case(self) -> super::to_server_message::MessageCase {
    let field_num = unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          0);
      ::protobuf::__internal::runtime::upb_Message_WhichOneofFieldNumber(
            self.raw_msg(), f)
    };
    unsafe {
      super::to_server_message::MessageCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ToServerMessageView` is `Sync` because it does not support mutation.
unsafe impl Sync for ToServerMessageView<'_> {}

// SAFETY:
// - `ToServerMessageView` is `Send` because while its alive a `ToServerMessageMut` cannot.
// - `ToServerMessageView` does not use thread-local data.
unsafe impl Send for ToServerMessageView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ToServerMessageView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ToServerMessageView<'msg> {}

impl<'msg> ::protobuf::AsView for ToServerMessageView<'msg> {
  type Proxied = ToServerMessage;
  fn as_view(&self) -> ::protobuf::View<'msg, ToServerMessage> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ToServerMessageView<'msg> {
  fn into_view<'shorter>(self) -> ToServerMessageView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ToServerMessage> for ToServerMessageView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ToServerMessage {
    let mut dst = ToServerMessage::new();
    let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(&mut dst, ::protobuf::__internal::Private);
    let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut dst, ::protobuf::__internal::Private);
    let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(&self, ::protobuf::__internal::Private);

    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst_raw,
      src_raw,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst_arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ToServerMessage> for ToServerMessageMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ToServerMessage {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for ToServerMessage {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
        ToServerMessageView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> ToServerMessageMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, ToServerMessage>::wrap_raw(msg, arena) };
        ToServerMessageMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ToServerMessageMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ToServerMessage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ToServerMessageMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ToServerMessageMut<'msg> {
  type Message = ToServerMessage;
}

impl ::std::fmt::Debug for ToServerMessageMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for ToServerMessageMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> ToServerMessageMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ToServerMessage>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ToServerMessage> {
    self.inner
  }

  pub fn to_owned(&self) -> ToServerMessage {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // login_request: optional message Proto.PayloadLoginRequest
  pub fn has_login_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_login_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn login_request_opt(&self) -> ::protobuf::Optional<super::PayloadLoginRequestView<'_>> {
        ::protobuf::Optional::new(self.login_request(), self.has_login_request())
  }
  pub fn login_request(&self) -> super::PayloadLoginRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLoginRequestView::new(::protobuf::__internal::Private, inner)
  }
  pub fn login_request_mut(&mut self) -> super::PayloadLoginRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.arena()
       ).unwrap()
     };
     super::PayloadLoginRequestMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_login_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLoginRequest>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        0, child_ptr
      );
    }
  }

  // logout_request: optional message Proto.PayloadLogoutRequest
  pub fn has_logout_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_logout_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn logout_request_opt(&self) -> ::protobuf::Optional<super::PayloadLogoutRequestView<'_>> {
        ::protobuf::Optional::new(self.logout_request(), self.has_logout_request())
  }
  pub fn logout_request(&self) -> super::PayloadLogoutRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLogoutRequestView::new(::protobuf::__internal::Private, inner)
  }
  pub fn logout_request_mut(&mut self) -> super::PayloadLogoutRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.arena()
       ).unwrap()
     };
     super::PayloadLogoutRequestMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_logout_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLogoutRequest>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        1, child_ptr
      );
    }
  }

  // signup_request: optional message Proto.PayloadSignupRequest
  pub fn has_signup_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_signup_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn signup_request_opt(&self) -> ::protobuf::Optional<super::PayloadSignupRequestView<'_>> {
        ::protobuf::Optional::new(self.signup_request(), self.has_signup_request())
  }
  pub fn signup_request(&self) -> super::PayloadSignupRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadSignupRequestView::new(::protobuf::__internal::Private, inner)
  }
  pub fn signup_request_mut(&mut self) -> super::PayloadSignupRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.arena()
       ).unwrap()
     };
     super::PayloadSignupRequestMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_signup_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadSignupRequest>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        2, child_ptr
      );
    }
  }

  // start_game: optional message Proto.PayloadLobbyStartGameRequest
  pub fn has_start_game(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_start_game(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn start_game_opt(&self) -> ::protobuf::Optional<super::PayloadLobbyStartGameRequestView<'_>> {
        ::protobuf::Optional::new(self.start_game(), self.has_start_game())
  }
  pub fn start_game(&self) -> super::PayloadLobbyStartGameRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLobbyStartGameRequestView::new(::protobuf::__internal::Private, inner)
  }
  pub fn start_game_mut(&mut self) -> super::PayloadLobbyStartGameRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.arena()
       ).unwrap()
     };
     super::PayloadLobbyStartGameRequestMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_start_game(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLobbyStartGameRequest>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        3, child_ptr
      );
    }
  }

  // end_game: optional message Proto.PayloadLobbyEndGameRequest
  pub fn has_end_game(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_end_game(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn end_game_opt(&self) -> ::protobuf::Optional<super::PayloadLobbyEndGameRequestView<'_>> {
        ::protobuf::Optional::new(self.end_game(), self.has_end_game())
  }
  pub fn end_game(&self) -> super::PayloadLobbyEndGameRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLobbyEndGameRequestView::new(::protobuf::__internal::Private, inner)
  }
  pub fn end_game_mut(&mut self) -> super::PayloadLobbyEndGameRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.arena()
       ).unwrap()
     };
     super::PayloadLobbyEndGameRequestMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_end_game(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLobbyEndGameRequest>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        4, child_ptr
      );
    }
  }

  // player_zone_enter_complete: optional message Proto.PayloadPlayerZoneEnterComplete
  pub fn has_player_zone_enter_complete(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_player_zone_enter_complete(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn player_zone_enter_complete_opt(&self) -> ::protobuf::Optional<super::PayloadPlayerZoneEnterCompleteView<'_>> {
        ::protobuf::Optional::new(self.player_zone_enter_complete(), self.has_player_zone_enter_complete())
  }
  pub fn player_zone_enter_complete(&self) -> super::PayloadPlayerZoneEnterCompleteView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadPlayerZoneEnterCompleteView::new(::protobuf::__internal::Private, inner)
  }
  pub fn player_zone_enter_complete_mut(&mut self) -> super::PayloadPlayerZoneEnterCompleteMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.arena()
       ).unwrap()
     };
     super::PayloadPlayerZoneEnterCompleteMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_player_zone_enter_complete(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadPlayerZoneEnterComplete>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        5, child_ptr
      );
    }
  }

  // transform_sync: optional message Proto.PayloadTransformSync
  pub fn has_transform_sync(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_transform_sync(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn transform_sync_opt(&self) -> ::protobuf::Optional<super::PayloadTransformSyncView<'_>> {
        ::protobuf::Optional::new(self.transform_sync(), self.has_transform_sync())
  }
  pub fn transform_sync(&self) -> super::PayloadTransformSyncView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTransformSyncView::new(::protobuf::__internal::Private, inner)
  }
  pub fn transform_sync_mut(&mut self) -> super::PayloadTransformSyncMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.arena()
       ).unwrap()
     };
     super::PayloadTransformSyncMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_transform_sync(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadTransformSync>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        6, child_ptr
      );
    }
  }

  // play_action: optional message Proto.PayloadPlayAction
  pub fn has_play_action(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_play_action(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn play_action_opt(&self) -> ::protobuf::Optional<super::PayloadPlayActionView<'_>> {
        ::protobuf::Optional::new(self.play_action(), self.has_play_action())
  }
  pub fn play_action(&self) -> super::PayloadPlayActionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadPlayActionView::new(::protobuf::__internal::Private, inner)
  }
  pub fn play_action_mut(&mut self) -> super::PayloadPlayActionMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.arena()
       ).unwrap()
     };
     super::PayloadPlayActionMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_play_action(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadPlayAction>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        7, child_ptr
      );
    }
  }

  // text_message: optional message Proto.PayloadTextMessage
  pub fn has_text_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_text_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn text_message_opt(&self) -> ::protobuf::Optional<super::PayloadTextMessageView<'_>> {
        ::protobuf::Optional::new(self.text_message(), self.has_text_message())
  }
  pub fn text_message(&self) -> super::PayloadTextMessageView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTextMessageView::new(::protobuf::__internal::Private, inner)
  }
  pub fn text_message_mut(&mut self) -> super::PayloadTextMessageMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.arena()
       ).unwrap()
     };
     super::PayloadTextMessageMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_text_message(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadTextMessage>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        8, child_ptr
      );
    }
  }

  pub fn message(&self) -> super::to_server_message::MessageOneof<'_> {
    match &self.message_case() {
      super::to_server_message::MessageCase::LoginRequest =>
          super::to_server_message::MessageOneof::LoginRequest(self.login_request()),
      super::to_server_message::MessageCase::LogoutRequest =>
          super::to_server_message::MessageOneof::LogoutRequest(self.logout_request()),
      super::to_server_message::MessageCase::SignupRequest =>
          super::to_server_message::MessageOneof::SignupRequest(self.signup_request()),
      super::to_server_message::MessageCase::StartGame =>
          super::to_server_message::MessageOneof::StartGame(self.start_game()),
      super::to_server_message::MessageCase::EndGame =>
          super::to_server_message::MessageOneof::EndGame(self.end_game()),
      super::to_server_message::MessageCase::PlayerZoneEnterComplete =>
          super::to_server_message::MessageOneof::PlayerZoneEnterComplete(self.player_zone_enter_complete()),
      super::to_server_message::MessageCase::TransformSync =>
          super::to_server_message::MessageOneof::TransformSync(self.transform_sync()),
      super::to_server_message::MessageCase::PlayAction =>
          super::to_server_message::MessageOneof::PlayAction(self.play_action()),
      super::to_server_message::MessageCase::TextMessage =>
          super::to_server_message::MessageOneof::TextMessage(self.text_message()),
      _ => super::to_server_message::MessageOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn message_case(&self) -> super::to_server_message::MessageCase {
    let field_num = unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          0);
      ::protobuf::__internal::runtime::upb_Message_WhichOneofFieldNumber(
            self.raw_msg(), f)
    };
    unsafe {
      super::to_server_message::MessageCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ToServerMessageMut` does not perform any shared mutation.
// - `ToServerMessageMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ToServerMessageMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ToServerMessageMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ToServerMessageMut<'msg> {}

impl<'msg> ::protobuf::AsView for ToServerMessageMut<'msg> {
  type Proxied = ToServerMessage;
  fn as_view(&self) -> ::protobuf::View<'_, ToServerMessage> {
    ToServerMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ToServerMessageMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ToServerMessage>
  where
      'msg: 'shorter {
    ToServerMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for ToServerMessageMut<'msg> {
  type MutProxied = ToServerMessage;
  fn as_mut(&mut self) -> ToServerMessageMut<'msg> {
    ToServerMessageMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ToServerMessageMut<'msg> {
  fn into_mut<'shorter>(self) -> ToServerMessageMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ToServerMessage {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ToServerMessage> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> ToServerMessageView {
    ToServerMessageView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> ToServerMessageMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    ToServerMessageMut::new(::protobuf::__internal::Private, inner)
  }

  // login_request: optional message Proto.PayloadLoginRequest
  pub fn has_login_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_login_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn login_request_opt(&self) -> ::protobuf::Optional<super::PayloadLoginRequestView<'_>> {
        ::protobuf::Optional::new(self.login_request(), self.has_login_request())
  }
  pub fn login_request(&self) -> super::PayloadLoginRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLoginRequestView::new(::protobuf::__internal::Private, inner)
  }
  pub fn login_request_mut(&mut self) -> super::PayloadLoginRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.arena()
       ).unwrap()
     };
     super::PayloadLoginRequestMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_login_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLoginRequest>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        0, child_ptr
      );
    }
  }

  // logout_request: optional message Proto.PayloadLogoutRequest
  pub fn has_logout_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_logout_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn logout_request_opt(&self) -> ::protobuf::Optional<super::PayloadLogoutRequestView<'_>> {
        ::protobuf::Optional::new(self.logout_request(), self.has_logout_request())
  }
  pub fn logout_request(&self) -> super::PayloadLogoutRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLogoutRequestView::new(::protobuf::__internal::Private, inner)
  }
  pub fn logout_request_mut(&mut self) -> super::PayloadLogoutRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.arena()
       ).unwrap()
     };
     super::PayloadLogoutRequestMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_logout_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLogoutRequest>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        1, child_ptr
      );
    }
  }

  // signup_request: optional message Proto.PayloadSignupRequest
  pub fn has_signup_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_signup_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn signup_request_opt(&self) -> ::protobuf::Optional<super::PayloadSignupRequestView<'_>> {
        ::protobuf::Optional::new(self.signup_request(), self.has_signup_request())
  }
  pub fn signup_request(&self) -> super::PayloadSignupRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadSignupRequestView::new(::protobuf::__internal::Private, inner)
  }
  pub fn signup_request_mut(&mut self) -> super::PayloadSignupRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.arena()
       ).unwrap()
     };
     super::PayloadSignupRequestMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_signup_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadSignupRequest>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        2, child_ptr
      );
    }
  }

  // start_game: optional message Proto.PayloadLobbyStartGameRequest
  pub fn has_start_game(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_start_game(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn start_game_opt(&self) -> ::protobuf::Optional<super::PayloadLobbyStartGameRequestView<'_>> {
        ::protobuf::Optional::new(self.start_game(), self.has_start_game())
  }
  pub fn start_game(&self) -> super::PayloadLobbyStartGameRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLobbyStartGameRequestView::new(::protobuf::__internal::Private, inner)
  }
  pub fn start_game_mut(&mut self) -> super::PayloadLobbyStartGameRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.arena()
       ).unwrap()
     };
     super::PayloadLobbyStartGameRequestMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_start_game(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLobbyStartGameRequest>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        3, child_ptr
      );
    }
  }

  // end_game: optional message Proto.PayloadLobbyEndGameRequest
  pub fn has_end_game(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_end_game(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn end_game_opt(&self) -> ::protobuf::Optional<super::PayloadLobbyEndGameRequestView<'_>> {
        ::protobuf::Optional::new(self.end_game(), self.has_end_game())
  }
  pub fn end_game(&self) -> super::PayloadLobbyEndGameRequestView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLobbyEndGameRequestView::new(::protobuf::__internal::Private, inner)
  }
  pub fn end_game_mut(&mut self) -> super::PayloadLobbyEndGameRequestMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.arena()
       ).unwrap()
     };
     super::PayloadLobbyEndGameRequestMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_end_game(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLobbyEndGameRequest>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        4, child_ptr
      );
    }
  }

  // player_zone_enter_complete: optional message Proto.PayloadPlayerZoneEnterComplete
  pub fn has_player_zone_enter_complete(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_player_zone_enter_complete(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn player_zone_enter_complete_opt(&self) -> ::protobuf::Optional<super::PayloadPlayerZoneEnterCompleteView<'_>> {
        ::protobuf::Optional::new(self.player_zone_enter_complete(), self.has_player_zone_enter_complete())
  }
  pub fn player_zone_enter_complete(&self) -> super::PayloadPlayerZoneEnterCompleteView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadPlayerZoneEnterCompleteView::new(::protobuf::__internal::Private, inner)
  }
  pub fn player_zone_enter_complete_mut(&mut self) -> super::PayloadPlayerZoneEnterCompleteMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.arena()
       ).unwrap()
     };
     super::PayloadPlayerZoneEnterCompleteMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_player_zone_enter_complete(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadPlayerZoneEnterComplete>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        5, child_ptr
      );
    }
  }

  // transform_sync: optional message Proto.PayloadTransformSync
  pub fn has_transform_sync(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_transform_sync(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn transform_sync_opt(&self) -> ::protobuf::Optional<super::PayloadTransformSyncView<'_>> {
        ::protobuf::Optional::new(self.transform_sync(), self.has_transform_sync())
  }
  pub fn transform_sync(&self) -> super::PayloadTransformSyncView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTransformSyncView::new(::protobuf::__internal::Private, inner)
  }
  pub fn transform_sync_mut(&mut self) -> super::PayloadTransformSyncMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.arena()
       ).unwrap()
     };
     super::PayloadTransformSyncMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_transform_sync(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadTransformSync>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        6, child_ptr
      );
    }
  }

  // play_action: optional message Proto.PayloadPlayAction
  pub fn has_play_action(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_play_action(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn play_action_opt(&self) -> ::protobuf::Optional<super::PayloadPlayActionView<'_>> {
        ::protobuf::Optional::new(self.play_action(), self.has_play_action())
  }
  pub fn play_action(&self) -> super::PayloadPlayActionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadPlayActionView::new(::protobuf::__internal::Private, inner)
  }
  pub fn play_action_mut(&mut self) -> super::PayloadPlayActionMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.arena()
       ).unwrap()
     };
     super::PayloadPlayActionMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_play_action(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadPlayAction>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        7, child_ptr
      );
    }
  }

  // text_message: optional message Proto.PayloadTextMessage
  pub fn has_text_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_text_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn text_message_opt(&self) -> ::protobuf::Optional<super::PayloadTextMessageView<'_>> {
        ::protobuf::Optional::new(self.text_message(), self.has_text_message())
  }
  pub fn text_message(&self) -> super::PayloadTextMessageView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTextMessageView::new(::protobuf::__internal::Private, inner)
  }
  pub fn text_message_mut(&mut self) -> super::PayloadTextMessageMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.arena()
       ).unwrap()
     };
     super::PayloadTextMessageMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_text_message(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadTextMessage>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        8, child_ptr
      );
    }
  }

  pub fn message(&self) -> super::to_server_message::MessageOneof<'_> {
    match &self.message_case() {
      super::to_server_message::MessageCase::LoginRequest =>
          super::to_server_message::MessageOneof::LoginRequest(self.login_request()),
      super::to_server_message::MessageCase::LogoutRequest =>
          super::to_server_message::MessageOneof::LogoutRequest(self.logout_request()),
      super::to_server_message::MessageCase::SignupRequest =>
          super::to_server_message::MessageOneof::SignupRequest(self.signup_request()),
      super::to_server_message::MessageCase::StartGame =>
          super::to_server_message::MessageOneof::StartGame(self.start_game()),
      super::to_server_message::MessageCase::EndGame =>
          super::to_server_message::MessageOneof::EndGame(self.end_game()),
      super::to_server_message::MessageCase::PlayerZoneEnterComplete =>
          super::to_server_message::MessageOneof::PlayerZoneEnterComplete(self.player_zone_enter_complete()),
      super::to_server_message::MessageCase::TransformSync =>
          super::to_server_message::MessageOneof::TransformSync(self.transform_sync()),
      super::to_server_message::MessageCase::PlayAction =>
          super::to_server_message::MessageOneof::PlayAction(self.play_action()),
      super::to_server_message::MessageCase::TextMessage =>
          super::to_server_message::MessageOneof::TextMessage(self.text_message()),
      _ => super::to_server_message::MessageOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn message_case(&self) -> super::to_server_message::MessageCase {
    let field_num = unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          0);
      ::protobuf::__internal::runtime::upb_Message_WhichOneofFieldNumber(
            self.raw_msg(), f)
    };
    unsafe {
      super::to_server_message::MessageCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ToServerMessage

impl ::std::ops::Drop for ToServerMessage {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ToServerMessage {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ToServerMessage {
  type Proxied = Self;
  fn as_view(&self) -> ToServerMessageView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ToServerMessage {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ToServerMessageMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ToServerMessage {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__ToServerMessage_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$333333333^!|#|$|%|&|(|)|*|+".as_ptr(),
              28,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
        <super::PayloadLoginRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadLogoutRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadSignupRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadLobbyStartGameRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadLobbyEndGameRequest as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadPlayerZoneEnterComplete as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadTransformSync as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadPlayAction as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadTextMessage as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__ToServerMessage_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__ToServerMessage_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ToServerMessage {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ToServerMessageView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <ToServerMessage as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ToServerMessageMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <ToServerMessage as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ToServerMessage {
  type Msg = ToServerMessage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ToServerMessage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ToServerMessage {
  type Msg = ToServerMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ToServerMessage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ToServerMessageMut<'_> {
  type Msg = ToServerMessage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ToServerMessage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ToServerMessageMut<'_> {
  type Msg = ToServerMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ToServerMessage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ToServerMessageView<'_> {
  type Msg = ToServerMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ToServerMessage> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ToServerMessageMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod to_server_message {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum MessageOneof<'msg> {
  LoginRequest(::protobuf::View<'msg, super::super::PayloadLoginRequest>) = 1,
  LogoutRequest(::protobuf::View<'msg, super::super::PayloadLogoutRequest>) = 2,
  SignupRequest(::protobuf::View<'msg, super::super::PayloadSignupRequest>) = 3,
  StartGame(::protobuf::View<'msg, super::super::PayloadLobbyStartGameRequest>) = 4,
  EndGame(::protobuf::View<'msg, super::super::PayloadLobbyEndGameRequest>) = 5,
  PlayerZoneEnterComplete(::protobuf::View<'msg, super::super::PayloadPlayerZoneEnterComplete>) = 6,
  TransformSync(::protobuf::View<'msg, super::super::PayloadTransformSync>) = 7,
  PlayAction(::protobuf::View<'msg, super::super::PayloadPlayAction>) = 8,
  TextMessage(::protobuf::View<'msg, super::super::PayloadTextMessage>) = 9,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum MessageCase {
  LoginRequest = 1,
  LogoutRequest = 2,
  SignupRequest = 3,
  StartGame = 4,
  EndGame = 5,
  PlayerZoneEnterComplete = 6,
  TransformSync = 7,
  PlayAction = 8,
  TextMessage = 9,

  not_set = 0
}

impl MessageCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<MessageCase> {
    match v {
      0 => Some(MessageCase::not_set),
      1 => Some(MessageCase::LoginRequest),
      2 => Some(MessageCase::LogoutRequest),
      3 => Some(MessageCase::SignupRequest),
      4 => Some(MessageCase::StartGame),
      5 => Some(MessageCase::EndGame),
      6 => Some(MessageCase::PlayerZoneEnterComplete),
      7 => Some(MessageCase::TransformSync),
      8 => Some(MessageCase::PlayAction),
      9 => Some(MessageCase::TextMessage),
      _ => None
    }
  }
}
}  // pub mod to_server_message

// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for ToServerMessage {}
impl<'a> ::protobuf::MessageMutInterop<'a> for ToServerMessageMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for ToServerMessageView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__ToClientMessage_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct ToClientMessage {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ToClientMessage>
}

impl ::protobuf::Message for ToClientMessage {}

impl ::std::default::Default for ToClientMessage {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for ToClientMessage {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for ToClientMessage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for ToClientMessage {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `ToClientMessage` is `Sync` because it does not implement interior mutability.
//    Neither does `ToClientMessageMut`.
unsafe impl Sync for ToClientMessage {}

// SAFETY:
// - `ToClientMessage` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ToClientMessage {}

impl ::protobuf::Proxied for ToClientMessage {
  type View<'msg> = ToClientMessageView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ToClientMessage {}

impl ::protobuf::MutProxied for ToClientMessage {
  type Mut<'msg> = ToClientMessageMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ToClientMessageView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ToClientMessage>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ToClientMessageView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ToClientMessageView<'msg> {
  type Message = ToClientMessage;
}

impl ::std::fmt::Debug for ToClientMessageView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for ToClientMessageView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for ToClientMessageView<'_> {
  fn default() -> ToClientMessageView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    ToClientMessageView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> ToClientMessageView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ToClientMessage>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> ToClientMessage {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // logout_response: optional message Proto.PayloadLogoutResponse
  pub fn has_logout_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn logout_response_opt(self) -> ::protobuf::Optional<super::PayloadLogoutResponseView<'msg>> {
        ::protobuf::Optional::new(self.logout_response(), self.has_logout_response())
  }
  pub fn logout_response(self) -> super::PayloadLogoutResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLogoutResponseView::new(::protobuf::__internal::Private, inner)
  }

  // signup_response: optional message Proto.PayloadSignupResponse
  pub fn has_signup_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn signup_response_opt(self) -> ::protobuf::Optional<super::PayloadSignupResponseView<'msg>> {
        ::protobuf::Optional::new(self.signup_response(), self.has_signup_response())
  }
  pub fn signup_response(self) -> super::PayloadSignupResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadSignupResponseView::new(::protobuf::__internal::Private, inner)
  }

  // lobby_enter_response: optional message Proto.PayloadLobbyEnterResponse
  pub fn has_lobby_enter_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn lobby_enter_response_opt(self) -> ::protobuf::Optional<super::PayloadLobbyEnterResponseView<'msg>> {
        ::protobuf::Optional::new(self.lobby_enter_response(), self.has_lobby_enter_response())
  }
  pub fn lobby_enter_response(self) -> super::PayloadLobbyEnterResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLobbyEnterResponseView::new(::protobuf::__internal::Private, inner)
  }

  // start_game_response: optional message Proto.PayloadLobbyStartGameResponse
  pub fn has_start_game_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn start_game_response_opt(self) -> ::protobuf::Optional<super::PayloadLobbyStartGameResponseView<'msg>> {
        ::protobuf::Optional::new(self.start_game_response(), self.has_start_game_response())
  }
  pub fn start_game_response(self) -> super::PayloadLobbyStartGameResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLobbyStartGameResponseView::new(::protobuf::__internal::Private, inner)
  }

  // end_game_response: optional message Proto.PayloadLobbyEndGameResponse
  pub fn has_end_game_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn end_game_response_opt(self) -> ::protobuf::Optional<super::PayloadLobbyEndGameResponseView<'msg>> {
        ::protobuf::Optional::new(self.end_game_response(), self.has_end_game_response())
  }
  pub fn end_game_response(self) -> super::PayloadLobbyEndGameResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLobbyEndGameResponseView::new(::protobuf::__internal::Private, inner)
  }

  // client_initializer_data: optional message Proto.PayloadClientInitializerData
  pub fn has_client_initializer_data(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn client_initializer_data_opt(self) -> ::protobuf::Optional<super::PayloadClientInitializerDataView<'msg>> {
        ::protobuf::Optional::new(self.client_initializer_data(), self.has_client_initializer_data())
  }
  pub fn client_initializer_data(self) -> super::PayloadClientInitializerDataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadClientInitializerDataView::new(::protobuf::__internal::Private, inner)
  }

  // transform_sync: optional message Proto.PayloadTransformSync
  pub fn has_transform_sync(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn transform_sync_opt(self) -> ::protobuf::Optional<super::PayloadTransformSyncView<'msg>> {
        ::protobuf::Optional::new(self.transform_sync(), self.has_transform_sync())
  }
  pub fn transform_sync(self) -> super::PayloadTransformSyncView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTransformSyncView::new(::protobuf::__internal::Private, inner)
  }

  // play_action: optional message Proto.PayloadPlayAction
  pub fn has_play_action(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn play_action_opt(self) -> ::protobuf::Optional<super::PayloadPlayActionView<'msg>> {
        ::protobuf::Optional::new(self.play_action(), self.has_play_action())
  }
  pub fn play_action(self) -> super::PayloadPlayActionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadPlayActionView::new(::protobuf::__internal::Private, inner)
  }

  // entity_damaged: optional message Proto.PayloadEntityDamaged
  pub fn has_entity_damaged(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn entity_damaged_opt(self) -> ::protobuf::Optional<super::PayloadEntityDamagedView<'msg>> {
        ::protobuf::Optional::new(self.entity_damaged(), self.has_entity_damaged())
  }
  pub fn entity_damaged(self) -> super::PayloadEntityDamagedView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEntityDamagedView::new(::protobuf::__internal::Private, inner)
  }

  // enemy_spawn: optional message Proto.PayloadEnemySpawn
  pub fn has_enemy_spawn(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn enemy_spawn_opt(self) -> ::protobuf::Optional<super::PayloadEnemySpawnView<'msg>> {
        ::protobuf::Optional::new(self.enemy_spawn(), self.has_enemy_spawn())
  }
  pub fn enemy_spawn(self) -> super::PayloadEnemySpawnView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEnemySpawnView::new(::protobuf::__internal::Private, inner)
  }

  // enemy_despawn: optional message Proto.PayloadEnemyDespawn
  pub fn has_enemy_despawn(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn enemy_despawn_opt(self) -> ::protobuf::Optional<super::PayloadEnemyDespawnView<'msg>> {
        ::protobuf::Optional::new(self.enemy_despawn(), self.has_enemy_despawn())
  }
  pub fn enemy_despawn(self) -> super::PayloadEnemyDespawnView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEnemyDespawnView::new(::protobuf::__internal::Private, inner)
  }

  // zone_enter_notification: optional message Proto.PayloadZoneEnterNotification
  pub fn has_zone_enter_notification(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn zone_enter_notification_opt(self) -> ::protobuf::Optional<super::PayloadZoneEnterNotificationView<'msg>> {
        ::protobuf::Optional::new(self.zone_enter_notification(), self.has_zone_enter_notification())
  }
  pub fn zone_enter_notification(self) -> super::PayloadZoneEnterNotificationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadZoneEnterNotificationView::new(::protobuf::__internal::Private, inner)
  }

  // zone_exit_notification: optional message Proto.PayloadZoneExitNotification
  pub fn has_zone_exit_notification(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn zone_exit_notification_opt(self) -> ::protobuf::Optional<super::PayloadZoneExitNotificationView<'msg>> {
        ::protobuf::Optional::new(self.zone_exit_notification(), self.has_zone_exit_notification())
  }
  pub fn zone_exit_notification(self) -> super::PayloadZoneExitNotificationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadZoneExitNotificationView::new(::protobuf::__internal::Private, inner)
  }

  // text_message: optional message Proto.PayloadTextMessage
  pub fn has_text_message(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn text_message_opt(self) -> ::protobuf::Optional<super::PayloadTextMessageView<'msg>> {
        ::protobuf::Optional::new(self.text_message(), self.has_text_message())
  }
  pub fn text_message(self) -> super::PayloadTextMessageView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTextMessageView::new(::protobuf::__internal::Private, inner)
  }

  pub fn message(self) -> super::to_client_message::MessageOneof<'msg> {
    match self.message_case() {
      super::to_client_message::MessageCase::LogoutResponse =>
          super::to_client_message::MessageOneof::LogoutResponse(self.logout_response()),
      super::to_client_message::MessageCase::SignupResponse =>
          super::to_client_message::MessageOneof::SignupResponse(self.signup_response()),
      super::to_client_message::MessageCase::LobbyEnterResponse =>
          super::to_client_message::MessageOneof::LobbyEnterResponse(self.lobby_enter_response()),
      super::to_client_message::MessageCase::StartGameResponse =>
          super::to_client_message::MessageOneof::StartGameResponse(self.start_game_response()),
      super::to_client_message::MessageCase::EndGameResponse =>
          super::to_client_message::MessageOneof::EndGameResponse(self.end_game_response()),
      super::to_client_message::MessageCase::ClientInitializerData =>
          super::to_client_message::MessageOneof::ClientInitializerData(self.client_initializer_data()),
      super::to_client_message::MessageCase::TransformSync =>
          super::to_client_message::MessageOneof::TransformSync(self.transform_sync()),
      super::to_client_message::MessageCase::PlayAction =>
          super::to_client_message::MessageOneof::PlayAction(self.play_action()),
      super::to_client_message::MessageCase::EntityDamaged =>
          super::to_client_message::MessageOneof::EntityDamaged(self.entity_damaged()),
      super::to_client_message::MessageCase::EnemySpawn =>
          super::to_client_message::MessageOneof::EnemySpawn(self.enemy_spawn()),
      super::to_client_message::MessageCase::EnemyDespawn =>
          super::to_client_message::MessageOneof::EnemyDespawn(self.enemy_despawn()),
      super::to_client_message::MessageCase::ZoneEnterNotification =>
          super::to_client_message::MessageOneof::ZoneEnterNotification(self.zone_enter_notification()),
      super::to_client_message::MessageCase::ZoneExitNotification =>
          super::to_client_message::MessageOneof::ZoneExitNotification(self.zone_exit_notification()),
      super::to_client_message::MessageCase::TextMessage =>
          super::to_client_message::MessageOneof::TextMessage(self.text_message()),
      _ => super::to_client_message::MessageOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn message_case(self) -> super::to_client_message::MessageCase {
    let field_num = unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          0);
      ::protobuf::__internal::runtime::upb_Message_WhichOneofFieldNumber(
            self.raw_msg(), f)
    };
    unsafe {
      super::to_client_message::MessageCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ToClientMessageView` is `Sync` because it does not support mutation.
unsafe impl Sync for ToClientMessageView<'_> {}

// SAFETY:
// - `ToClientMessageView` is `Send` because while its alive a `ToClientMessageMut` cannot.
// - `ToClientMessageView` does not use thread-local data.
unsafe impl Send for ToClientMessageView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ToClientMessageView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ToClientMessageView<'msg> {}

impl<'msg> ::protobuf::AsView for ToClientMessageView<'msg> {
  type Proxied = ToClientMessage;
  fn as_view(&self) -> ::protobuf::View<'msg, ToClientMessage> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ToClientMessageView<'msg> {
  fn into_view<'shorter>(self) -> ToClientMessageView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ToClientMessage> for ToClientMessageView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ToClientMessage {
    let mut dst = ToClientMessage::new();
    let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(&mut dst, ::protobuf::__internal::Private);
    let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut dst, ::protobuf::__internal::Private);
    let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(&self, ::protobuf::__internal::Private);

    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst_raw,
      src_raw,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst_arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ToClientMessage> for ToClientMessageMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ToClientMessage {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for ToClientMessage {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
        ToClientMessageView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> ToClientMessageMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, ToClientMessage>::wrap_raw(msg, arena) };
        ToClientMessageMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ToClientMessageMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ToClientMessage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ToClientMessageMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ToClientMessageMut<'msg> {
  type Message = ToClientMessage;
}

impl ::std::fmt::Debug for ToClientMessageMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for ToClientMessageMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> ToClientMessageMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ToClientMessage>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ToClientMessage> {
    self.inner
  }

  pub fn to_owned(&self) -> ToClientMessage {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // logout_response: optional message Proto.PayloadLogoutResponse
  pub fn has_logout_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_logout_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn logout_response_opt(&self) -> ::protobuf::Optional<super::PayloadLogoutResponseView<'_>> {
        ::protobuf::Optional::new(self.logout_response(), self.has_logout_response())
  }
  pub fn logout_response(&self) -> super::PayloadLogoutResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLogoutResponseView::new(::protobuf::__internal::Private, inner)
  }
  pub fn logout_response_mut(&mut self) -> super::PayloadLogoutResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.arena()
       ).unwrap()
     };
     super::PayloadLogoutResponseMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_logout_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLogoutResponse>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        0, child_ptr
      );
    }
  }

  // signup_response: optional message Proto.PayloadSignupResponse
  pub fn has_signup_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_signup_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn signup_response_opt(&self) -> ::protobuf::Optional<super::PayloadSignupResponseView<'_>> {
        ::protobuf::Optional::new(self.signup_response(), self.has_signup_response())
  }
  pub fn signup_response(&self) -> super::PayloadSignupResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadSignupResponseView::new(::protobuf::__internal::Private, inner)
  }
  pub fn signup_response_mut(&mut self) -> super::PayloadSignupResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.arena()
       ).unwrap()
     };
     super::PayloadSignupResponseMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_signup_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadSignupResponse>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        1, child_ptr
      );
    }
  }

  // lobby_enter_response: optional message Proto.PayloadLobbyEnterResponse
  pub fn has_lobby_enter_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lobby_enter_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lobby_enter_response_opt(&self) -> ::protobuf::Optional<super::PayloadLobbyEnterResponseView<'_>> {
        ::protobuf::Optional::new(self.lobby_enter_response(), self.has_lobby_enter_response())
  }
  pub fn lobby_enter_response(&self) -> super::PayloadLobbyEnterResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLobbyEnterResponseView::new(::protobuf::__internal::Private, inner)
  }
  pub fn lobby_enter_response_mut(&mut self) -> super::PayloadLobbyEnterResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.arena()
       ).unwrap()
     };
     super::PayloadLobbyEnterResponseMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_lobby_enter_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLobbyEnterResponse>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        2, child_ptr
      );
    }
  }

  // start_game_response: optional message Proto.PayloadLobbyStartGameResponse
  pub fn has_start_game_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_start_game_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn start_game_response_opt(&self) -> ::protobuf::Optional<super::PayloadLobbyStartGameResponseView<'_>> {
        ::protobuf::Optional::new(self.start_game_response(), self.has_start_game_response())
  }
  pub fn start_game_response(&self) -> super::PayloadLobbyStartGameResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLobbyStartGameResponseView::new(::protobuf::__internal::Private, inner)
  }
  pub fn start_game_response_mut(&mut self) -> super::PayloadLobbyStartGameResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.arena()
       ).unwrap()
     };
     super::PayloadLobbyStartGameResponseMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_start_game_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLobbyStartGameResponse>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        3, child_ptr
      );
    }
  }

  // end_game_response: optional message Proto.PayloadLobbyEndGameResponse
  pub fn has_end_game_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_end_game_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn end_game_response_opt(&self) -> ::protobuf::Optional<super::PayloadLobbyEndGameResponseView<'_>> {
        ::protobuf::Optional::new(self.end_game_response(), self.has_end_game_response())
  }
  pub fn end_game_response(&self) -> super::PayloadLobbyEndGameResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLobbyEndGameResponseView::new(::protobuf::__internal::Private, inner)
  }
  pub fn end_game_response_mut(&mut self) -> super::PayloadLobbyEndGameResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.arena()
       ).unwrap()
     };
     super::PayloadLobbyEndGameResponseMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_end_game_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLobbyEndGameResponse>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        4, child_ptr
      );
    }
  }

  // client_initializer_data: optional message Proto.PayloadClientInitializerData
  pub fn has_client_initializer_data(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_client_initializer_data(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn client_initializer_data_opt(&self) -> ::protobuf::Optional<super::PayloadClientInitializerDataView<'_>> {
        ::protobuf::Optional::new(self.client_initializer_data(), self.has_client_initializer_data())
  }
  pub fn client_initializer_data(&self) -> super::PayloadClientInitializerDataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadClientInitializerDataView::new(::protobuf::__internal::Private, inner)
  }
  pub fn client_initializer_data_mut(&mut self) -> super::PayloadClientInitializerDataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.arena()
       ).unwrap()
     };
     super::PayloadClientInitializerDataMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_client_initializer_data(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadClientInitializerData>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        5, child_ptr
      );
    }
  }

  // transform_sync: optional message Proto.PayloadTransformSync
  pub fn has_transform_sync(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_transform_sync(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn transform_sync_opt(&self) -> ::protobuf::Optional<super::PayloadTransformSyncView<'_>> {
        ::protobuf::Optional::new(self.transform_sync(), self.has_transform_sync())
  }
  pub fn transform_sync(&self) -> super::PayloadTransformSyncView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTransformSyncView::new(::protobuf::__internal::Private, inner)
  }
  pub fn transform_sync_mut(&mut self) -> super::PayloadTransformSyncMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.arena()
       ).unwrap()
     };
     super::PayloadTransformSyncMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_transform_sync(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadTransformSync>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        6, child_ptr
      );
    }
  }

  // play_action: optional message Proto.PayloadPlayAction
  pub fn has_play_action(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_play_action(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn play_action_opt(&self) -> ::protobuf::Optional<super::PayloadPlayActionView<'_>> {
        ::protobuf::Optional::new(self.play_action(), self.has_play_action())
  }
  pub fn play_action(&self) -> super::PayloadPlayActionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadPlayActionView::new(::protobuf::__internal::Private, inner)
  }
  pub fn play_action_mut(&mut self) -> super::PayloadPlayActionMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.arena()
       ).unwrap()
     };
     super::PayloadPlayActionMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_play_action(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadPlayAction>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        7, child_ptr
      );
    }
  }

  // entity_damaged: optional message Proto.PayloadEntityDamaged
  pub fn has_entity_damaged(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_entity_damaged(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn entity_damaged_opt(&self) -> ::protobuf::Optional<super::PayloadEntityDamagedView<'_>> {
        ::protobuf::Optional::new(self.entity_damaged(), self.has_entity_damaged())
  }
  pub fn entity_damaged(&self) -> super::PayloadEntityDamagedView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEntityDamagedView::new(::protobuf::__internal::Private, inner)
  }
  pub fn entity_damaged_mut(&mut self) -> super::PayloadEntityDamagedMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.arena()
       ).unwrap()
     };
     super::PayloadEntityDamagedMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_entity_damaged(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadEntityDamaged>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        8, child_ptr
      );
    }
  }

  // enemy_spawn: optional message Proto.PayloadEnemySpawn
  pub fn has_enemy_spawn(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_enemy_spawn(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn enemy_spawn_opt(&self) -> ::protobuf::Optional<super::PayloadEnemySpawnView<'_>> {
        ::protobuf::Optional::new(self.enemy_spawn(), self.has_enemy_spawn())
  }
  pub fn enemy_spawn(&self) -> super::PayloadEnemySpawnView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEnemySpawnView::new(::protobuf::__internal::Private, inner)
  }
  pub fn enemy_spawn_mut(&mut self) -> super::PayloadEnemySpawnMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         9, self.arena()
       ).unwrap()
     };
     super::PayloadEnemySpawnMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_enemy_spawn(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadEnemySpawn>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        9, child_ptr
      );
    }
  }

  // enemy_despawn: optional message Proto.PayloadEnemyDespawn
  pub fn has_enemy_despawn(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_enemy_despawn(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn enemy_despawn_opt(&self) -> ::protobuf::Optional<super::PayloadEnemyDespawnView<'_>> {
        ::protobuf::Optional::new(self.enemy_despawn(), self.has_enemy_despawn())
  }
  pub fn enemy_despawn(&self) -> super::PayloadEnemyDespawnView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEnemyDespawnView::new(::protobuf::__internal::Private, inner)
  }
  pub fn enemy_despawn_mut(&mut self) -> super::PayloadEnemyDespawnMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         10, self.arena()
       ).unwrap()
     };
     super::PayloadEnemyDespawnMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_enemy_despawn(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadEnemyDespawn>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        10, child_ptr
      );
    }
  }

  // zone_enter_notification: optional message Proto.PayloadZoneEnterNotification
  pub fn has_zone_enter_notification(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_zone_enter_notification(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn zone_enter_notification_opt(&self) -> ::protobuf::Optional<super::PayloadZoneEnterNotificationView<'_>> {
        ::protobuf::Optional::new(self.zone_enter_notification(), self.has_zone_enter_notification())
  }
  pub fn zone_enter_notification(&self) -> super::PayloadZoneEnterNotificationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadZoneEnterNotificationView::new(::protobuf::__internal::Private, inner)
  }
  pub fn zone_enter_notification_mut(&mut self) -> super::PayloadZoneEnterNotificationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         11, self.arena()
       ).unwrap()
     };
     super::PayloadZoneEnterNotificationMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_zone_enter_notification(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadZoneEnterNotification>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        11, child_ptr
      );
    }
  }

  // zone_exit_notification: optional message Proto.PayloadZoneExitNotification
  pub fn has_zone_exit_notification(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_zone_exit_notification(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn zone_exit_notification_opt(&self) -> ::protobuf::Optional<super::PayloadZoneExitNotificationView<'_>> {
        ::protobuf::Optional::new(self.zone_exit_notification(), self.has_zone_exit_notification())
  }
  pub fn zone_exit_notification(&self) -> super::PayloadZoneExitNotificationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadZoneExitNotificationView::new(::protobuf::__internal::Private, inner)
  }
  pub fn zone_exit_notification_mut(&mut self) -> super::PayloadZoneExitNotificationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.arena()
       ).unwrap()
     };
     super::PayloadZoneExitNotificationMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_zone_exit_notification(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadZoneExitNotification>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        12, child_ptr
      );
    }
  }

  // text_message: optional message Proto.PayloadTextMessage
  pub fn has_text_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_text_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn text_message_opt(&self) -> ::protobuf::Optional<super::PayloadTextMessageView<'_>> {
        ::protobuf::Optional::new(self.text_message(), self.has_text_message())
  }
  pub fn text_message(&self) -> super::PayloadTextMessageView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTextMessageView::new(::protobuf::__internal::Private, inner)
  }
  pub fn text_message_mut(&mut self) -> super::PayloadTextMessageMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         13, self.arena()
       ).unwrap()
     };
     super::PayloadTextMessageMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_text_message(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadTextMessage>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        13, child_ptr
      );
    }
  }

  pub fn message(&self) -> super::to_client_message::MessageOneof<'_> {
    match &self.message_case() {
      super::to_client_message::MessageCase::LogoutResponse =>
          super::to_client_message::MessageOneof::LogoutResponse(self.logout_response()),
      super::to_client_message::MessageCase::SignupResponse =>
          super::to_client_message::MessageOneof::SignupResponse(self.signup_response()),
      super::to_client_message::MessageCase::LobbyEnterResponse =>
          super::to_client_message::MessageOneof::LobbyEnterResponse(self.lobby_enter_response()),
      super::to_client_message::MessageCase::StartGameResponse =>
          super::to_client_message::MessageOneof::StartGameResponse(self.start_game_response()),
      super::to_client_message::MessageCase::EndGameResponse =>
          super::to_client_message::MessageOneof::EndGameResponse(self.end_game_response()),
      super::to_client_message::MessageCase::ClientInitializerData =>
          super::to_client_message::MessageOneof::ClientInitializerData(self.client_initializer_data()),
      super::to_client_message::MessageCase::TransformSync =>
          super::to_client_message::MessageOneof::TransformSync(self.transform_sync()),
      super::to_client_message::MessageCase::PlayAction =>
          super::to_client_message::MessageOneof::PlayAction(self.play_action()),
      super::to_client_message::MessageCase::EntityDamaged =>
          super::to_client_message::MessageOneof::EntityDamaged(self.entity_damaged()),
      super::to_client_message::MessageCase::EnemySpawn =>
          super::to_client_message::MessageOneof::EnemySpawn(self.enemy_spawn()),
      super::to_client_message::MessageCase::EnemyDespawn =>
          super::to_client_message::MessageOneof::EnemyDespawn(self.enemy_despawn()),
      super::to_client_message::MessageCase::ZoneEnterNotification =>
          super::to_client_message::MessageOneof::ZoneEnterNotification(self.zone_enter_notification()),
      super::to_client_message::MessageCase::ZoneExitNotification =>
          super::to_client_message::MessageOneof::ZoneExitNotification(self.zone_exit_notification()),
      super::to_client_message::MessageCase::TextMessage =>
          super::to_client_message::MessageOneof::TextMessage(self.text_message()),
      _ => super::to_client_message::MessageOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn message_case(&self) -> super::to_client_message::MessageCase {
    let field_num = unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          0);
      ::protobuf::__internal::runtime::upb_Message_WhichOneofFieldNumber(
            self.raw_msg(), f)
    };
    unsafe {
      super::to_client_message::MessageCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ToClientMessageMut` does not perform any shared mutation.
// - `ToClientMessageMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ToClientMessageMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ToClientMessageMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ToClientMessageMut<'msg> {}

impl<'msg> ::protobuf::AsView for ToClientMessageMut<'msg> {
  type Proxied = ToClientMessage;
  fn as_view(&self) -> ::protobuf::View<'_, ToClientMessage> {
    ToClientMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ToClientMessageMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ToClientMessage>
  where
      'msg: 'shorter {
    ToClientMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for ToClientMessageMut<'msg> {
  type MutProxied = ToClientMessage;
  fn as_mut(&mut self) -> ToClientMessageMut<'msg> {
    ToClientMessageMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ToClientMessageMut<'msg> {
  fn into_mut<'shorter>(self) -> ToClientMessageMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ToClientMessage {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ToClientMessage> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> ToClientMessageView {
    ToClientMessageView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> ToClientMessageMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    ToClientMessageMut::new(::protobuf::__internal::Private, inner)
  }

  // logout_response: optional message Proto.PayloadLogoutResponse
  pub fn has_logout_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_logout_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn logout_response_opt(&self) -> ::protobuf::Optional<super::PayloadLogoutResponseView<'_>> {
        ::protobuf::Optional::new(self.logout_response(), self.has_logout_response())
  }
  pub fn logout_response(&self) -> super::PayloadLogoutResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLogoutResponseView::new(::protobuf::__internal::Private, inner)
  }
  pub fn logout_response_mut(&mut self) -> super::PayloadLogoutResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.arena()
       ).unwrap()
     };
     super::PayloadLogoutResponseMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_logout_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLogoutResponse>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        0, child_ptr
      );
    }
  }

  // signup_response: optional message Proto.PayloadSignupResponse
  pub fn has_signup_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_signup_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn signup_response_opt(&self) -> ::protobuf::Optional<super::PayloadSignupResponseView<'_>> {
        ::protobuf::Optional::new(self.signup_response(), self.has_signup_response())
  }
  pub fn signup_response(&self) -> super::PayloadSignupResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadSignupResponseView::new(::protobuf::__internal::Private, inner)
  }
  pub fn signup_response_mut(&mut self) -> super::PayloadSignupResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.arena()
       ).unwrap()
     };
     super::PayloadSignupResponseMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_signup_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadSignupResponse>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        1, child_ptr
      );
    }
  }

  // lobby_enter_response: optional message Proto.PayloadLobbyEnterResponse
  pub fn has_lobby_enter_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_lobby_enter_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn lobby_enter_response_opt(&self) -> ::protobuf::Optional<super::PayloadLobbyEnterResponseView<'_>> {
        ::protobuf::Optional::new(self.lobby_enter_response(), self.has_lobby_enter_response())
  }
  pub fn lobby_enter_response(&self) -> super::PayloadLobbyEnterResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLobbyEnterResponseView::new(::protobuf::__internal::Private, inner)
  }
  pub fn lobby_enter_response_mut(&mut self) -> super::PayloadLobbyEnterResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.arena()
       ).unwrap()
     };
     super::PayloadLobbyEnterResponseMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_lobby_enter_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLobbyEnterResponse>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        2, child_ptr
      );
    }
  }

  // start_game_response: optional message Proto.PayloadLobbyStartGameResponse
  pub fn has_start_game_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_start_game_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn start_game_response_opt(&self) -> ::protobuf::Optional<super::PayloadLobbyStartGameResponseView<'_>> {
        ::protobuf::Optional::new(self.start_game_response(), self.has_start_game_response())
  }
  pub fn start_game_response(&self) -> super::PayloadLobbyStartGameResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLobbyStartGameResponseView::new(::protobuf::__internal::Private, inner)
  }
  pub fn start_game_response_mut(&mut self) -> super::PayloadLobbyStartGameResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.arena()
       ).unwrap()
     };
     super::PayloadLobbyStartGameResponseMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_start_game_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLobbyStartGameResponse>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        3, child_ptr
      );
    }
  }

  // end_game_response: optional message Proto.PayloadLobbyEndGameResponse
  pub fn has_end_game_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_end_game_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn end_game_response_opt(&self) -> ::protobuf::Optional<super::PayloadLobbyEndGameResponseView<'_>> {
        ::protobuf::Optional::new(self.end_game_response(), self.has_end_game_response())
  }
  pub fn end_game_response(&self) -> super::PayloadLobbyEndGameResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadLobbyEndGameResponseView::new(::protobuf::__internal::Private, inner)
  }
  pub fn end_game_response_mut(&mut self) -> super::PayloadLobbyEndGameResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.arena()
       ).unwrap()
     };
     super::PayloadLobbyEndGameResponseMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_end_game_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadLobbyEndGameResponse>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        4, child_ptr
      );
    }
  }

  // client_initializer_data: optional message Proto.PayloadClientInitializerData
  pub fn has_client_initializer_data(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_client_initializer_data(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn client_initializer_data_opt(&self) -> ::protobuf::Optional<super::PayloadClientInitializerDataView<'_>> {
        ::protobuf::Optional::new(self.client_initializer_data(), self.has_client_initializer_data())
  }
  pub fn client_initializer_data(&self) -> super::PayloadClientInitializerDataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadClientInitializerDataView::new(::protobuf::__internal::Private, inner)
  }
  pub fn client_initializer_data_mut(&mut self) -> super::PayloadClientInitializerDataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.arena()
       ).unwrap()
     };
     super::PayloadClientInitializerDataMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_client_initializer_data(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadClientInitializerData>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        5, child_ptr
      );
    }
  }

  // transform_sync: optional message Proto.PayloadTransformSync
  pub fn has_transform_sync(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_transform_sync(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn transform_sync_opt(&self) -> ::protobuf::Optional<super::PayloadTransformSyncView<'_>> {
        ::protobuf::Optional::new(self.transform_sync(), self.has_transform_sync())
  }
  pub fn transform_sync(&self) -> super::PayloadTransformSyncView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTransformSyncView::new(::protobuf::__internal::Private, inner)
  }
  pub fn transform_sync_mut(&mut self) -> super::PayloadTransformSyncMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.arena()
       ).unwrap()
     };
     super::PayloadTransformSyncMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_transform_sync(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadTransformSync>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        6, child_ptr
      );
    }
  }

  // play_action: optional message Proto.PayloadPlayAction
  pub fn has_play_action(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_play_action(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn play_action_opt(&self) -> ::protobuf::Optional<super::PayloadPlayActionView<'_>> {
        ::protobuf::Optional::new(self.play_action(), self.has_play_action())
  }
  pub fn play_action(&self) -> super::PayloadPlayActionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadPlayActionView::new(::protobuf::__internal::Private, inner)
  }
  pub fn play_action_mut(&mut self) -> super::PayloadPlayActionMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.arena()
       ).unwrap()
     };
     super::PayloadPlayActionMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_play_action(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadPlayAction>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        7, child_ptr
      );
    }
  }

  // entity_damaged: optional message Proto.PayloadEntityDamaged
  pub fn has_entity_damaged(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_entity_damaged(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn entity_damaged_opt(&self) -> ::protobuf::Optional<super::PayloadEntityDamagedView<'_>> {
        ::protobuf::Optional::new(self.entity_damaged(), self.has_entity_damaged())
  }
  pub fn entity_damaged(&self) -> super::PayloadEntityDamagedView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEntityDamagedView::new(::protobuf::__internal::Private, inner)
  }
  pub fn entity_damaged_mut(&mut self) -> super::PayloadEntityDamagedMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.arena()
       ).unwrap()
     };
     super::PayloadEntityDamagedMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_entity_damaged(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadEntityDamaged>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        8, child_ptr
      );
    }
  }

  // enemy_spawn: optional message Proto.PayloadEnemySpawn
  pub fn has_enemy_spawn(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_enemy_spawn(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn enemy_spawn_opt(&self) -> ::protobuf::Optional<super::PayloadEnemySpawnView<'_>> {
        ::protobuf::Optional::new(self.enemy_spawn(), self.has_enemy_spawn())
  }
  pub fn enemy_spawn(&self) -> super::PayloadEnemySpawnView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEnemySpawnView::new(::protobuf::__internal::Private, inner)
  }
  pub fn enemy_spawn_mut(&mut self) -> super::PayloadEnemySpawnMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         9, self.arena()
       ).unwrap()
     };
     super::PayloadEnemySpawnMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_enemy_spawn(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadEnemySpawn>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        9, child_ptr
      );
    }
  }

  // enemy_despawn: optional message Proto.PayloadEnemyDespawn
  pub fn has_enemy_despawn(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_enemy_despawn(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn enemy_despawn_opt(&self) -> ::protobuf::Optional<super::PayloadEnemyDespawnView<'_>> {
        ::protobuf::Optional::new(self.enemy_despawn(), self.has_enemy_despawn())
  }
  pub fn enemy_despawn(&self) -> super::PayloadEnemyDespawnView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadEnemyDespawnView::new(::protobuf::__internal::Private, inner)
  }
  pub fn enemy_despawn_mut(&mut self) -> super::PayloadEnemyDespawnMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         10, self.arena()
       ).unwrap()
     };
     super::PayloadEnemyDespawnMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_enemy_despawn(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadEnemyDespawn>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        10, child_ptr
      );
    }
  }

  // zone_enter_notification: optional message Proto.PayloadZoneEnterNotification
  pub fn has_zone_enter_notification(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_zone_enter_notification(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn zone_enter_notification_opt(&self) -> ::protobuf::Optional<super::PayloadZoneEnterNotificationView<'_>> {
        ::protobuf::Optional::new(self.zone_enter_notification(), self.has_zone_enter_notification())
  }
  pub fn zone_enter_notification(&self) -> super::PayloadZoneEnterNotificationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadZoneEnterNotificationView::new(::protobuf::__internal::Private, inner)
  }
  pub fn zone_enter_notification_mut(&mut self) -> super::PayloadZoneEnterNotificationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         11, self.arena()
       ).unwrap()
     };
     super::PayloadZoneEnterNotificationMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_zone_enter_notification(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadZoneEnterNotification>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        11, child_ptr
      );
    }
  }

  // zone_exit_notification: optional message Proto.PayloadZoneExitNotification
  pub fn has_zone_exit_notification(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_zone_exit_notification(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn zone_exit_notification_opt(&self) -> ::protobuf::Optional<super::PayloadZoneExitNotificationView<'_>> {
        ::protobuf::Optional::new(self.zone_exit_notification(), self.has_zone_exit_notification())
  }
  pub fn zone_exit_notification(&self) -> super::PayloadZoneExitNotificationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadZoneExitNotificationView::new(::protobuf::__internal::Private, inner)
  }
  pub fn zone_exit_notification_mut(&mut self) -> super::PayloadZoneExitNotificationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.arena()
       ).unwrap()
     };
     super::PayloadZoneExitNotificationMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_zone_exit_notification(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadZoneExitNotification>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        12, child_ptr
      );
    }
  }

  // text_message: optional message Proto.PayloadTextMessage
  pub fn has_text_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_text_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn text_message_opt(&self) -> ::protobuf::Optional<super::PayloadTextMessageView<'_>> {
        ::protobuf::Optional::new(self.text_message(), self.has_text_message())
  }
  pub fn text_message(&self) -> super::PayloadTextMessageView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::PayloadTextMessageView::new(::protobuf::__internal::Private, inner)
  }
  pub fn text_message_mut(&mut self) -> super::PayloadTextMessageMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         13, self.arena()
       ).unwrap()
     };
     super::PayloadTextMessageMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_text_message(&mut self,
    val: impl ::protobuf::IntoProxied<super::PayloadTextMessage>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        13, child_ptr
      );
    }
  }

  pub fn message(&self) -> super::to_client_message::MessageOneof<'_> {
    match &self.message_case() {
      super::to_client_message::MessageCase::LogoutResponse =>
          super::to_client_message::MessageOneof::LogoutResponse(self.logout_response()),
      super::to_client_message::MessageCase::SignupResponse =>
          super::to_client_message::MessageOneof::SignupResponse(self.signup_response()),
      super::to_client_message::MessageCase::LobbyEnterResponse =>
          super::to_client_message::MessageOneof::LobbyEnterResponse(self.lobby_enter_response()),
      super::to_client_message::MessageCase::StartGameResponse =>
          super::to_client_message::MessageOneof::StartGameResponse(self.start_game_response()),
      super::to_client_message::MessageCase::EndGameResponse =>
          super::to_client_message::MessageOneof::EndGameResponse(self.end_game_response()),
      super::to_client_message::MessageCase::ClientInitializerData =>
          super::to_client_message::MessageOneof::ClientInitializerData(self.client_initializer_data()),
      super::to_client_message::MessageCase::TransformSync =>
          super::to_client_message::MessageOneof::TransformSync(self.transform_sync()),
      super::to_client_message::MessageCase::PlayAction =>
          super::to_client_message::MessageOneof::PlayAction(self.play_action()),
      super::to_client_message::MessageCase::EntityDamaged =>
          super::to_client_message::MessageOneof::EntityDamaged(self.entity_damaged()),
      super::to_client_message::MessageCase::EnemySpawn =>
          super::to_client_message::MessageOneof::EnemySpawn(self.enemy_spawn()),
      super::to_client_message::MessageCase::EnemyDespawn =>
          super::to_client_message::MessageOneof::EnemyDespawn(self.enemy_despawn()),
      super::to_client_message::MessageCase::ZoneEnterNotification =>
          super::to_client_message::MessageOneof::ZoneEnterNotification(self.zone_enter_notification()),
      super::to_client_message::MessageCase::ZoneExitNotification =>
          super::to_client_message::MessageOneof::ZoneExitNotification(self.zone_exit_notification()),
      super::to_client_message::MessageCase::TextMessage =>
          super::to_client_message::MessageOneof::TextMessage(self.text_message()),
      _ => super::to_client_message::MessageOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn message_case(&self) -> super::to_client_message::MessageCase {
    let field_num = unsafe {
      let f = ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
          0);
      ::protobuf::__internal::runtime::upb_Message_WhichOneofFieldNumber(
            self.raw_msg(), f)
    };
    unsafe {
      super::to_client_message::MessageCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ToClientMessage

impl ::std::ops::Drop for ToClientMessage {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ToClientMessage {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ToClientMessage {
  type Proxied = Self;
  fn as_view(&self) -> ToClientMessageView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ToClientMessage {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ToClientMessageMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ToClientMessage {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__ToClientMessage_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$a33333333333333^#|$|%|&|(|)|*|+|,|-|.|/|0|1".as_ptr(),
              44,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
        <super::PayloadLogoutResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadSignupResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadLobbyEnterResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadLobbyStartGameResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadLobbyEndGameResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadClientInitializerData as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadTransformSync as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadPlayAction as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadEntityDamaged as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadEnemySpawn as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadEnemyDespawn as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadZoneEnterNotification as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadZoneExitNotification as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::PayloadTextMessage as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__ToClientMessage_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__ToClientMessage_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ToClientMessage {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ToClientMessageView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <ToClientMessage as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ToClientMessageMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <ToClientMessage as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ToClientMessage {
  type Msg = ToClientMessage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ToClientMessage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ToClientMessage {
  type Msg = ToClientMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ToClientMessage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ToClientMessageMut<'_> {
  type Msg = ToClientMessage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ToClientMessage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ToClientMessageMut<'_> {
  type Msg = ToClientMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ToClientMessage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ToClientMessageView<'_> {
  type Msg = ToClientMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ToClientMessage> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ToClientMessageMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod to_client_message {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum MessageOneof<'msg> {
  LogoutResponse(::protobuf::View<'msg, super::super::PayloadLogoutResponse>) = 2,
  SignupResponse(::protobuf::View<'msg, super::super::PayloadSignupResponse>) = 3,
  LobbyEnterResponse(::protobuf::View<'msg, super::super::PayloadLobbyEnterResponse>) = 4,
  StartGameResponse(::protobuf::View<'msg, super::super::PayloadLobbyStartGameResponse>) = 5,
  EndGameResponse(::protobuf::View<'msg, super::super::PayloadLobbyEndGameResponse>) = 6,
  ClientInitializerData(::protobuf::View<'msg, super::super::PayloadClientInitializerData>) = 7,
  TransformSync(::protobuf::View<'msg, super::super::PayloadTransformSync>) = 8,
  PlayAction(::protobuf::View<'msg, super::super::PayloadPlayAction>) = 9,
  EntityDamaged(::protobuf::View<'msg, super::super::PayloadEntityDamaged>) = 10,
  EnemySpawn(::protobuf::View<'msg, super::super::PayloadEnemySpawn>) = 11,
  EnemyDespawn(::protobuf::View<'msg, super::super::PayloadEnemyDespawn>) = 12,
  ZoneEnterNotification(::protobuf::View<'msg, super::super::PayloadZoneEnterNotification>) = 13,
  ZoneExitNotification(::protobuf::View<'msg, super::super::PayloadZoneExitNotification>) = 14,
  TextMessage(::protobuf::View<'msg, super::super::PayloadTextMessage>) = 15,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum MessageCase {
  LogoutResponse = 2,
  SignupResponse = 3,
  LobbyEnterResponse = 4,
  StartGameResponse = 5,
  EndGameResponse = 6,
  ClientInitializerData = 7,
  TransformSync = 8,
  PlayAction = 9,
  EntityDamaged = 10,
  EnemySpawn = 11,
  EnemyDespawn = 12,
  ZoneEnterNotification = 13,
  ZoneExitNotification = 14,
  TextMessage = 15,

  not_set = 0
}

impl MessageCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<MessageCase> {
    match v {
      0 => Some(MessageCase::not_set),
      2 => Some(MessageCase::LogoutResponse),
      3 => Some(MessageCase::SignupResponse),
      4 => Some(MessageCase::LobbyEnterResponse),
      5 => Some(MessageCase::StartGameResponse),
      6 => Some(MessageCase::EndGameResponse),
      7 => Some(MessageCase::ClientInitializerData),
      8 => Some(MessageCase::TransformSync),
      9 => Some(MessageCase::PlayAction),
      10 => Some(MessageCase::EntityDamaged),
      11 => Some(MessageCase::EnemySpawn),
      12 => Some(MessageCase::EnemyDespawn),
      13 => Some(MessageCase::ZoneEnterNotification),
      14 => Some(MessageCase::ZoneExitNotification),
      15 => Some(MessageCase::TextMessage),
      _ => None
    }
  }
}
}  // pub mod to_client_message

// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for ToClientMessage {}
impl<'a> ::protobuf::MessageMutInterop<'a> for ToClientMessageMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for ToClientMessageView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

