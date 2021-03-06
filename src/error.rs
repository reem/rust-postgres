//! Postgres errors

use std::collections::HashMap;
use std::io;
use std::fmt;

use openssl::ssl::error;
use phf::PhfMap;

use PostgresResult;
use types::PostgresType;

macro_rules! make_errors(
    ($($code:expr => $error:ident),+) => (
        /// SQLSTATE error codes
        #[deriving(PartialEq, Eq, Clone, Show)]
        #[allow(missing_doc)]
        pub enum PostgresSqlState {
            $($error,)+
            UnknownSqlState(String)
        }

        static STATE_MAP: PhfMap<&'static str, PostgresSqlState> = phf_map!(
            $($code => $error),+
        );

        impl PostgresSqlState {
            #[doc(hidden)]
            pub fn from_code(s: &str) -> PostgresSqlState {
                match STATE_MAP.find_equiv(&s) {
                    Some(state) => state.clone(),
                    None => UnknownSqlState(s.into_string())
                }
            }
        }
    )
)

// From http://www.postgresql.org/docs/9.2/static/errcodes-appendix.html
make_errors!(
    // Class 00 — Successful Completion
    "00000" => SuccessfulCompletion,

    // Class 01 — Warning
    "01000" => Warning,
    "0100C" => DynamicResultSetsReturned,
    "01008" => ImplicitZeroBitPadding,
    "01003" => NullValueEliminatedInSetFunction,
    "01007" => PrivilegeNotGranted,
    "01006" => PrivilegeNotRevoked,
    "01004" => StringDataRightTruncationWarning,
    "01P01" => DeprecatedFeature,

    // Class 02 — No Data
    "02000" => NoData,
    "02001" => NoAdditionalDynamicResultSetsReturned,

    // Class 03 — SQL Statement Not Yet Complete
    "03000" => SqlStatementNotYetComplete,

    // Class 08 — Connection Exception
    "08000" => ConnectionException,
    "08003" => ConnectionDoesNotExist,
    "08006" => ConnectionFailure,
    "08001" => SqlclientUnableToEstablishSqlconnection,
    "08004" => SqlserverRejectedEstablishmentOfSqlconnection,
    "08007" => TransactionResolutionUnknown,
    "08P01" => ProtocolViolation,

    // Class 09 — Triggered Action Exception
    "09000" => TriggeredActionException,

    // Class 0A — Feature Not Supported
    "0A000" => FeatureNotSupported,

    // Class 0B — Invalid Transaction Initiation
    "0B000" => InvalidTransactionInitiation,

    // Class 0F — Locator Exception
    "0F000" => LocatorException,
    "0F001" => InvalidLocatorException,

    // Class 0L — Invalid Grantor
    "0L000" => InvalidGrantor,
    "0LP01" => InvalidGrantOperation,

    // Class 0P — Invalid Role Specification
    "0P000" => InvalidRoleSpecification,

    // Class 0Z — Diagnostics Exception
    "0Z000" => DiagnosticsException,
    "0Z002" => StackedDiagnosticsAccessedWithoutActiveHandler,

    // Class 20 — Case Not Found
    "20000" => CaseNotFound,

    // Class 21 — Cardinality Violation
    "21000" => CardinalityViolation,

    // Class 22 — Data Exception
    "22000" => DataException,
    "2202E" => ArraySubscriptError,
    "22021" => CharacterNotInRepertoire,
    "22008" => DatetimeFieldOverflow,
    "22012" => DivisionByZero,
    "22005" => ErrorInAssignment,
    "2200B" => EscapeCharacterConflict,
    "22022" => IndicatorOverflow,
    "22015" => IntervalFieldOverflow,
    "2201E" => InvalidArgumentForLogarithm,
    "22014" => InvalidArgumentForNtileFunction,
    "22016" => InvalidArgumentForNthValueFunction,
    "2201F" => InvalidArgumentForPowerFunction,
    "2201G" => InvalidArgumentForWidthBucketFunction,
    "22018" => InvalidCharacterValueForCast,
    "22007" => InvalidDatetimeFormat,
    "22019" => InvalidEscapeCharacter,
    "2200D" => InvalidEscapeOctet,
    "22025" => InvalidEscapeSequence,
    "22P06" => NonstandardUseOfEscapeCharacter,
    "22010" => InvalidIndicatorParameterValue,
    "22023" => InvalidParameterValue,
    "2201B" => InvalidRegularExpression,
    "2201W" => InvalidRowCountInLimitClause,
    "2201X" => InvalidRowCountInResultOffsetClause,
    "22009" => InvalidTimeZoneDisplacementValue,
    "2200C" => InvalidUseOfEscapeCharacter,
    "2200G" => MostSpecificTypeMismatch,
    "22004" => NullValueNotAllowedData,
    "22002" => NullValueNoIndicatorParameter,
    "22003" => NumericValueOutOfRange,
    "22026" => StringDataLengthMismatch,
    "22001" => StringDataRightTruncationException,
    "22011" => SubstringError,
    "22027" => TrimError,
    "22024" => UnterminatedCString,
    "2200F" => ZeroLengthCharacterString,
    "22P01" => FloatingPointException,
    "22P02" => InvalidTextRepresentation,
    "22P03" => InvalidBinaryRepresentation,
    "22P04" => BadCopyFileFormat,
    "22P05" => UntranslatableCharacter,
    "2200L" => NotAnXmlDocument,
    "2200M" => InvalidXmlDocument,
    "2200N" => InvalidXmlContent,
    "2200S" => InvalidXmlComment,
    "2200T" => InvalidXmlProcessingInstruction,

    // Class 23 — Integrity Constraint Violation
    "23000" => IntegrityConstraintViolation,
    "23001" => RestrictViolation,
    "23502" => NotNullViolation,
    "23503" => ForeignKeyViolation,
    "23505" => UniqueViolation,
    "23514" => CheckViolation,
    "32P01" => ExclusionViolation,

    // Class 24 — Invalid Cursor State
    "24000" => InvalidCursorState,

    // Class 25 — Invalid Transaction State
    "25000" => InvalidTransactionState,
    "25001" => ActiveSqlTransaction,
    "25002" => BranchTransactionAlreadyActive,
    "25008" => HeldCursorRequiresSameIsolationLevel,
    "25003" => InappropriateAccessModeForBranchTransaction,
    "25004" => InappropriateIsolationLevelForBranchTransaction,
    "25005" => NoActiveSqlTransactionForBranchTransaction,
    "25006" => ReadOnlySqlTransaction,
    "25007" => SchemaAndDataStatementMixingNotSupported,
    "25P01" => NoActiveSqlTransaction,
    "25P02" => InFailedSqlTransaction,

    // Class 26 — Invalid SQL Statement Name
    "26000" => InvalidSqlStatementName,

    // Class 27 — Triggered Data Change Violation
    "27000" => TriggeredDataChangeViolation,

    // Class 28 — Invalid Authorization Specification
    "28000" => InvalidAuthorizationSpecification,
    "28P01" => InvalidPassword,

    // Class 2B — Dependent Privilege Descriptors Still Exist
    "2B000" => DependentPrivilegeDescriptorsStillExist,
    "2BP01" => DependentObjectsStillExist,

    // Class 2D — Invalid Transaction Termination
    "2D000" => InvalidTransactionTermination,

    // Class 2F — SQL Routine Exception
    "2F000" => SqlRoutineException,
    "2F005" => FunctionExecutedNoReturnStatement,
    "2F002" => ModifyingSqlDataNotPermittedSqlRoutine,
    "2F003" => ProhibitedSqlStatementAttemptedSqlRoutine,
    "2F004" => ReadingSqlDataNotPermittedSqlRoutine,

    // Class 34 — Invalid Cursor Name
    "34000" => InvalidCursorName,

    // Class 38 — External Routine Exception
    "38000" => ExternalRoutineException,
    "38001" => ContainingSqlNotPermitted,
    "38002" => ModifyingSqlDataNotPermittedExternalRoutine,
    "38003" => ProhibitedSqlStatementAttemptedExternalRoutine,
    "38004" => ReadingSqlDataNotPermittedExternalRoutine,

    // Class 39 — External Routine Invocation Exception
    "39000" => ExternalRoutineInvocationException,
    "39001" => InvalidSqlstateReturned,
    "39004" => NullValueNotAllowedExternalRoutine,
    "39P01" => TriggerProtocolViolated,
    "39P02" => SrfProtocolViolated,

    // Class 3B — Savepoint Exception
    "3B000" => SavepointException,
    "3B001" => InvalidSavepointException,

    // Class 3D — Invalid Catalog Name
    "3D000" => InvalidCatalogName,

    // Class 3F — Invalid Schema Name
    "3F000" => InvalidSchemaName,

    // Class 40 — Transaction Rollback
    "40000" => TransactionRollback,
    "40002" => TransactionIntegrityConstraintViolation,
    "40001" => SerializationFailure,
    "40003" => StatementCompletionUnknown,
    "40P01" => DeadlockDetected,

    // Class 42 — Syntax Error or Access Rule Violation
    "42000" => SyntaxErrorOrAccessRuleViolation,
    "42601" => SyntaxError,
    "42501" => InsufficientPrivilege,
    "42846" => CannotCoerce,
    "42803" => GroupingError,
    "42P20" => WindowingError,
    "42P19" => InvalidRecursion,
    "42830" => InvalidForeignKey,
    "42602" => InvalidName,
    "42622" => NameTooLong,
    "42939" => ReservedName,
    "42804" => DatatypeMismatch,
    "42P18" => IndeterminateDatatype,
    "42P21" => CollationMismatch,
    "42P22" => IndeterminateCollation,
    "42809" => WrongObjectType,
    "42703" => UndefinedColumn,
    "42883" => UndefinedFunction,
    "42P01" => UndefinedTable,
    "42P02" => UndefinedParameter,
    "42704" => UndefinedObject,
    "42701" => DuplicateColumn,
    "42P03" => DuplicateCursor,
    "42P04" => DuplicateDatabase,
    "42723" => DuplicateFunction,
    "42P05" => DuplicatePreparedStatement,
    "42P06" => DuplicateSchema,
    "42P07" => DuplicateTable,
    "42712" => DuplicateAliaas,
    "42710" => DuplicateObject,
    "42702" => AmbiguousColumn,
    "42725" => AmbiguousFunction,
    "42P08" => AmbiguousParameter,
    "42P09" => AmbiguousAlias,
    "42P10" => InvalidColumnReference,
    "42611" => InvalidColumnDefinition,
    "42P11" => InvalidCursorDefinition,
    "42P12" => InvalidDatabaseDefinition,
    "42P13" => InvalidFunctionDefinition,
    "42P14" => InvalidPreparedStatementDefinition,
    "42P15" => InvalidSchemaDefinition,
    "42P16" => InvalidTableDefinition,
    "42P17" => InvalidObjectDefinition,

    // Class 44 — WITH CHECK OPTION Violation
    "44000" => WithCheckOptionViolation,

    // Class 53 — Insufficient Resources
    "53000" => InsufficientResources,
    "53100" => DiskFull,
    "53200" => OutOfMemory,
    "53300" => TooManyConnections,
    "53400" => ConfigurationLimitExceeded,

    // Class 54 — Program Limit Exceeded
    "54000" => ProgramLimitExceeded,
    "54001" => StatementTooComplex,
    "54011" => TooManyColumns,
    "54023" => TooManyArguments,

    // Class 55 — Object Not In Prerequisite State
    "55000" => ObjectNotInPrerequisiteState,
    "55006" => ObjectInUse,
    "55P02" => CantChangeRuntimeParam,
    "55P03" => LockNotAvailable,

    // Class 57 — Operator Intervention
    "57000" => OperatorIntervention,
    "57014" => QueryCanceled,
    "57P01" => AdminShutdown,
    "57P02" => CrashShutdown,
    "57P03" => CannotConnectNow,
    "57P04" => DatabaseDropped,

    // Class 58 — System Error
    "58000" => SystemError,
    "58030" => IoError,
    "58P01" => UndefinedFile,
    "58P02" => DuplicateFile,

    // Class F0 — Configuration File Error
    "F0000" => ConfigFileError,
    "F0001" => LockFileExists,

    // Class HV — Foreign Data Wrapper Error (SQL/MED)
    "HV000" => FdwError,
    "HV005" => FdwColumnNameNotFound,
    "HV002" => FdwDynamicParameterValueNeeded,
    "HV010" => FdwFunctionSequenceError,
    "HV021" => FdwInconsistentDescriptorInformation,
    "HV024" => FdwInvalidAttributeValue,
    "HV007" => FdwInvalidColumnName,
    "HV008" => FdwInvalidColumnNumber,
    "HV004" => FdwInvalidDataType,
    "HV006" => FdwInvalidDataTypeDescriptors,
    "HV091" => FdwInvalidDescriptorFieldIdentifier,
    "HV00B" => FdwInvalidHandle,
    "HV00C" => FdwInvalidOptionIndex,
    "HV00D" => FdwInvalidOptionName,
    "HV090" => FdwInvalidStringLengthOrBufferLength,
    "HV00A" => FdwInvalidStringFormat,
    "HV009" => FdwInvalidUseOfNullPointer,
    "HV014" => FdwTooManyHandles,
    "HV001" => FdwOutOfMemory,
    "HV00P" => FdwNoSchemas,
    "HV00J" => FdwOptionNameNotFound,
    "HV00K" => FdwReplyHandle,
    "HV00Q" => FdwSchemaNotFound,
    "HV00R" => FdwTableNotFound,
    "HV00L" => FdwUnableToCreateExcecution,
    "HV00M" => FdwUnableToCreateReply,
    "HV00N" => FdwUnableToEstablishConnection,

    // Class P0 — PL/pgSQL Error
    "P0000" => PlpgsqlError,
    "P0001" => RaiseException,
    "P0002" => NoDataFound,
    "P0003" => TooManyRows,

    // Class XX — Internal Error
    "XX000" => InternalError,
    "XX001" => DataCorrupted,
    "XX002" => IndexCorrupted
)

/// Reasons a new Postgres connection could fail
#[deriving(Clone, PartialEq, Eq)]
pub enum PostgresConnectError {
    /// The provided URL could not be parsed
    InvalidUrl(String),
    /// The URL was missing a user
    MissingUser,
    /// There was an error opening a socket to the server
    SocketError(io::IoError),
    /// An error from the Postgres server itself
    PgConnectDbError(PostgresDbError),
    /// A password was required but not provided in the URL
    MissingPassword,
    /// The Postgres server requested an authentication method not supported
    /// by the driver
    UnsupportedAuthentication,
    /// The Postgres server does not support SSL encryption
    NoSslSupport,
    /// There was an error initializing the SSL session
    SslError(error::SslError),
    /// There was an error communicating with the server
    PgConnectStreamError(io::IoError),
    /// The server sent an unexpected response
    PgConnectBadResponse,
}

impl fmt::Show for PostgresConnectError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvalidUrl(ref err) => write!(fmt, "Invalid URL: {}", err),
            MissingUser => write!(fmt, "User missing in URL"),
            SocketError(ref err) =>
                write!(fmt, "Unable to open connection to server: {}", err),
            PgConnectDbError(ref err) => err.fmt(fmt),
            MissingPassword =>
                write!(fmt, "The server requested a password but none was provided"),
            UnsupportedAuthentication =>
                write!(fmt, "The server requested an unsupported authentication method"),
            NoSslSupport =>
                write!(fmt, "The server does not support SSL"),
            SslError(ref err) =>
                write!(fmt, "Error initiating SSL session: {}", err),
            PgConnectStreamError(ref err) =>
                write!(fmt, "Error communicating with server: {}", err),
            PgConnectBadResponse =>
                write!(fmt, "The server returned an unexpected response"),
        }
    }
}

/// Represents the position of an error in a query
#[deriving(Clone, PartialEq, Eq)]
pub enum PostgresErrorPosition {
    /// A position in the original query
    Position(uint),
    /// A position in an internally generated query
    InternalPosition {
        /// The byte position
        pub position: uint,
        /// A query generated by the Postgres server
        pub query: String
    }
}

/// Encapsulates a Postgres error or notice.
#[deriving(Clone, PartialEq, Eq)]
pub struct PostgresDbError {
    /// The field contents are ERROR, FATAL, or PANIC (in an error message),
    /// or WARNING, NOTICE, DEBUG, INFO, or LOG (in a notice message), or a
    /// localized translation of one of these.
    pub severity: String,
    /// The SQLSTATE code for the error.
    pub code: PostgresSqlState,
    /// The primary human-readable error message. This should be accurate but
    /// terse (typically one line).
    pub message: String,
    /// An optional secondary error message carrying more detail about the
    /// problem. Might run to multiple lines.
    pub detail: Option<String>,
    /// An optional suggestion what to do about the problem. This is intended
    /// to differ from Detail in that it offers advice (potentially
    /// inappropriate) rather than hard facts. Might run to multiple lines.
    pub hint: Option<String>,
    /// An optional error cursor position into either the original query string
    /// or an internally generated query.
    pub position: Option<PostgresErrorPosition>,
    /// An indication of the context in which the error occurred. Presently
    /// this includes a call stack traceback of active procedural language
    /// functions and internally-generated queries. The trace is one entry per
    /// line, most recent first.
    pub where_: Option<String>,
    /// If the error was associated with a specific database object, the name
    /// of the schema containing that object, if any. (PostgreSQL 9.3+)
    pub schema: Option<String>,
    /// If the error was associated with a specific table, the name of the
    /// table. (Refer to the schema name field for the name of the table's
    /// schema.) (PostgreSQL 9.3+)
    pub table: Option<String>,
    /// If the error was associated with a specific table column, the name of
    /// the column. (Refer to the schema and table name fields to identify the
    /// table.) (PostgreSQL 9.3+)
    pub column: Option<String>,
    /// If the error was associated with a specific data type, the name of the
    /// data type. (Refer to the schema name field for the name of the data
    /// type's schema.) (PostgreSQL 9.3+)
    pub datatype: Option<String>,
    /// If the error was associated with a specific constraint, the name of the
    /// constraint. Refer to fields listed above for the associated table or
    /// domain. (For this purpose, indexes are treated as constraints, even if
    /// they weren't created with constraint syntax.) (PostgreSQL 9.3+)
    pub constraint: Option<String>,
    /// The file name of the source-code location where the error was reported.
    pub file: String,
    /// The line number of the source-code location where the error was
    /// reported.
    pub line: uint,
    /// The name of the source-code routine reporting the error.
    pub routine: String
}

impl PostgresDbError {
    #[doc(hidden)]
    pub fn new_raw(fields: Vec<(u8, String)>) -> Result<PostgresDbError, ()> {
        let mut map: HashMap<_, _> = fields.into_iter().collect();
        Ok(PostgresDbError {
            severity: try!(map.pop(&b'S').ok_or(())),
            code: PostgresSqlState::from_code(try!(map.pop(&b'C').ok_or(()))[]),
            message: try!(map.pop(&b'M').ok_or(())),
            detail: map.pop(&b'D'),
            hint: map.pop(&b'H'),
            position: match map.pop(&b'P') {
                Some(pos) => Some(Position(try!(from_str(pos[]).ok_or(())))),
                None => match map.pop(&b'p') {
                    Some(pos) => Some(InternalPosition {
                        position: try!(from_str(pos[]).ok_or(())),
                        query: try!(map.pop(&b'q').ok_or(()))
                    }),
                    None => None
                }
            },
            where_: map.pop(&b'W'),
            schema: map.pop(&b's'),
            table: map.pop(&b't'),
            column: map.pop(&b'c'),
            datatype: map.pop(&b'd'),
            constraint: map.pop(&b'n'),
            file: try!(map.pop(&b'F').ok_or(())),
            line: try!(map.pop(&b'L').and_then(|l| from_str(l[])).ok_or(())),
            routine: try!(map.pop(&b'R').ok_or(())),
        })
    }

    #[doc(hidden)]
    pub fn new_connect<T>(fields: Vec<(u8, String)>) -> Result<T, PostgresConnectError> {
        match PostgresDbError::new_raw(fields) {
            Ok(err) => Err(PgConnectDbError(err)),
            Err(()) => Err(PgConnectBadResponse),
        }
    }

    #[doc(hidden)]
    pub fn new<T>(fields: Vec<(u8, String)>) -> PostgresResult<T> {
        match PostgresDbError::new_raw(fields) {
            Ok(err) => Err(PgDbError(err)),
            Err(()) => Err(PgBadData),
        }
    }
}

impl fmt::Show for PostgresDbError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}: {}", self.severity, self.message)
    }
}

/// An error encountered when communicating with the Postgres server
#[deriving(Clone, PartialEq, Eq)]
pub enum PostgresError {
    /// An error reported by the Postgres server
    PgDbError(PostgresDbError),
    /// An error communicating with the Postgres server
    PgStreamError(io::IoError),
    /// The communication channel with the Postgres server has desynchronized
    /// due to an earlier communications error.
    PgStreamDesynchronized,
    /// A prepared statement was executed on a connection it does not belong to
    PgWrongConnection,
    /// An incorrect number of parameters were bound to a statement
    PgWrongParamCount {
        /// The expected number of parameters
        pub expected: uint,
        /// The actual number of parameters
        pub actual: uint,
    },
    /// An attempt was made to convert between incompatible Rust and Postgres
    /// types
    PgWrongType(PostgresType),
    /// An attempt was made to read from a column that does not exist
    PgInvalidColumn,
    /// A value was NULL but converted to a non-nullable Rust type
    PgWasNull,
    /// An attempt was made to prepare a statement or start a transaction on an
    /// object other than the active transaction
    PgWrongTransaction,
    /// The server returned an unexpected response
    PgBadResponse,
    /// The server provided data that the client could not parse
    PgBadData,
}

impl fmt::Show for PostgresError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PgDbError(ref err) => err.fmt(fmt),
            PgStreamError(ref err) => err.fmt(fmt),
            PgStreamDesynchronized =>
                write!(fmt, "Communication with the server has desynchronized due to an earlier \
                             IO error"),
            PgWrongConnection =>
                write!(fmt, "A statement was executed with a connection it was not prepared with"),
            PgWrongParamCount { expected, actual } =>
                write!(fmt, "Expected {} parameters but got {}", expected, actual),
            PgWrongType(ref ty) => write!(fmt, "Unexpected type {}", ty),
            PgInvalidColumn => write!(fmt, "Invalid column"),
            PgWasNull => write!(fmt, "The value was NULL"),
            PgWrongTransaction =>
                write!(fmt, "An attempt was made to prepare a statement or start a transaction on \
                             an object other than the active transaction"),
            PgBadResponse =>
                write!(fmt, "The server returned an unexpected response"),
            PgBadData =>
                write!(fmt, "The server provided data that the client could not parse"),
        }
    }
}
