use derive_more::From;
use either::Either;
use std::io;

use super::framed::DispatcherError;

/// Errors which can occur when attempting to handle mqtt connection.
#[derive(Debug)]
pub enum MqttError<E> {
    /// Publish handler service error
    Service(E),
    /// Protocol error
    Protocol(ProtocolError),
    /// Publish service readiness error
    PublishReadyError,
    /// Mqtt parse error
    Decode(DecodeError),
    /// Mqtt encode error
    Encode(EncodeError),
    /// Unexpected packet
    Unexpected(u8, &'static str),
    /// "SUBSCRIBE, UNSUBSCRIBE, and PUBLISH (in cases where QoS > 0) Control Packets MUST contain a non-zero 16-bit Packet Identifier [MQTT-2.3.1-1]."
    PacketIdRequired,
    /// Multiple in-flight publish packet with same package_id
    DuplicatedPacketId,
    /// Packet id of publish ack packet does not match of send publish packet
    PacketIdMismatch,
    /// Topic alias is greater than max topic alias
    MaxTopicAlias,
    /// Unknown topic alias
    UnknownTopicAlias,
    /// Keep alive timeout
    KeepAliveTimeout,
    /// Handshake timeout
    HandshakeTimeout,
    /// Peer disconnect
    Disconnected,
    /// Unexpected io error
    Io(io::Error),
}

/// Protocol level errors
#[derive(Debug)]
pub enum ProtocolError {
    /// Publish service readiness error
    PublishReadyError,
    /// Mqtt parse error
    Decode(DecodeError),
    /// Mqtt encode error
    Encode(EncodeError),
    /// Unexpected packet
    Unexpected(u8, &'static str),
    /// "SUBSCRIBE, UNSUBSCRIBE, and PUBLISH (in cases where QoS > 0) Control Packets MUST contain a non-zero 16-bit Packet Identifier [MQTT-2.3.1-1]."
    PacketIdRequired,
    /// Multiple in-flight publish packet with same package_id
    DuplicatedPacketId,
    /// Packet id of publish ack packet does not match of send publish packet
    PacketIdMismatch,
    /// Topic alias is greater than max topic alias
    MaxTopicAlias,
    /// Unknown topic alias
    UnknownTopicAlias,
    /// Keep alive timeout
    KeepAliveTimeout,
    /// Handshake timeout
    HandshakeTimeout,
    /// Peer disconnect
    Disconnected,
    /// Unexpected io error
    Io(io::Error),
}

impl<E> From<Either<E, ProtocolError>> for MqttError<E> {
    fn from(err: Either<E, ProtocolError>) -> Self {
        match err {
            Either::Left(e) => MqttError::Service(e),
            Either::Right(e) => MqttError::Protocol(e),
        }
    }
}

impl<E> From<DecodeError> for MqttError<E> {
    fn from(err: DecodeError) -> Self {
        MqttError::Decode(err)
    }
}

impl<E> From<Either<DecodeError, io::Error>> for MqttError<E> {
    fn from(err: Either<DecodeError, io::Error>) -> Self {
        match err {
            Either::Left(err) => MqttError::Decode(err),
            Either::Right(err) => MqttError::Io(err),
        }
    }
}

impl<E> From<Either<EncodeError, io::Error>> for MqttError<E> {
    fn from(err: Either<EncodeError, io::Error>) -> Self {
        match err {
            Either::Left(err) => MqttError::Encode(err),
            Either::Right(err) => MqttError::Io(err),
        }
    }
}

impl<E> From<EncodeError> for MqttError<E> {
    fn from(err: EncodeError) -> Self {
        MqttError::Encode(err)
    }
}

impl<E> From<io::Error> for MqttError<E> {
    fn from(err: io::Error) -> Self {
        MqttError::Io(err)
    }
}

impl<E> From<DispatcherError<crate::v3::codec::Codec>> for MqttError<E> {
    fn from(err: DispatcherError<crate::v3::codec::Codec>) -> Self {
        match err {
            DispatcherError::KeepAlive => MqttError::KeepAliveTimeout,
            DispatcherError::Encoder(err) => MqttError::Encode(err),
            DispatcherError::Decoder(err) => MqttError::Decode(err),
            DispatcherError::Io(err) => MqttError::Io(err),
        }
    }
}

impl<E> From<DispatcherError<crate::v5::codec::Codec>> for MqttError<E> {
    fn from(err: DispatcherError<crate::v5::codec::Codec>) -> Self {
        match err {
            DispatcherError::KeepAlive => MqttError::KeepAliveTimeout,
            DispatcherError::Encoder(err) => MqttError::Encode(err),
            DispatcherError::Decoder(err) => MqttError::Decode(err),
            DispatcherError::Io(err) => MqttError::Io(err),
        }
    }
}

impl From<DispatcherError<crate::v3::codec::Codec>> for ProtocolError {
    fn from(err: DispatcherError<crate::v3::codec::Codec>) -> Self {
        match err {
            DispatcherError::KeepAlive => ProtocolError::KeepAliveTimeout,
            DispatcherError::Encoder(err) => ProtocolError::Encode(err),
            DispatcherError::Decoder(err) => ProtocolError::Decode(err),
            DispatcherError::Io(err) => ProtocolError::Io(err),
        }
    }
}

impl From<DispatcherError<crate::v5::codec::Codec>> for ProtocolError {
    fn from(err: DispatcherError<crate::v5::codec::Codec>) -> Self {
        match err {
            DispatcherError::KeepAlive => ProtocolError::KeepAliveTimeout,
            DispatcherError::Encoder(err) => ProtocolError::Encode(err),
            DispatcherError::Decoder(err) => ProtocolError::Decode(err),
            DispatcherError::Io(err) => ProtocolError::Io(err),
        }
    }
}

#[derive(Debug, From)]
pub enum DecodeError {
    InvalidProtocol,
    InvalidLength,
    MalformedPacket,
    UnsupportedProtocolLevel,
    ConnectReservedFlagSet,
    ConnAckReservedFlagSet,
    InvalidClientId,
    UnsupportedPacketType,
    // MQTT v3 only
    PacketIdRequired,
    MaxSizeExceeded,
    Utf8Error(std::str::Utf8Error),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum EncodeError {
    InvalidLength,
    MalformedPacket,
    PacketIdRequired,
    UnsupportedVersion,
}

impl PartialEq for DecodeError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DecodeError::InvalidProtocol, DecodeError::InvalidProtocol) => true,
            (DecodeError::InvalidLength, DecodeError::InvalidLength) => true,
            (DecodeError::UnsupportedProtocolLevel, DecodeError::UnsupportedProtocolLevel) => {
                true
            }
            (DecodeError::ConnectReservedFlagSet, DecodeError::ConnectReservedFlagSet) => true,
            (DecodeError::ConnAckReservedFlagSet, DecodeError::ConnAckReservedFlagSet) => true,
            (DecodeError::InvalidClientId, DecodeError::InvalidClientId) => true,
            (DecodeError::UnsupportedPacketType, DecodeError::UnsupportedPacketType) => true,
            (DecodeError::PacketIdRequired, DecodeError::PacketIdRequired) => true,
            (DecodeError::MaxSizeExceeded, DecodeError::MaxSizeExceeded) => true,
            (DecodeError::MalformedPacket, DecodeError::MalformedPacket) => true,
            (DecodeError::Utf8Error(_), _) => false,
            _ => false,
        }
    }
}
