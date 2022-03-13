pipeline {
  agent {
    docker { image 'rust:latest' }
  }
  stages {
    stage('Build') {
      steps {
        sh 'cargo build'
        archiveArtifact('target/debug/typos-json-to-github-annotations')
      }
    }

    stage('Test') {
      steps {
        sh 'cargo test'
      }
    }

    stage('Clippy') {
      steps {
        sh '''
          rustup component add clippy
          cargo clippy --all
        '''
      }
      post {
        always {
          recordIssues tool: cargo()
        }
      }
    }

    stage('Rustfmt') {
      steps {
        sh 'cargo fmt --all -- check'
      }
      post {
        always {
          recordIssues tool: cargo()
        }
      }
    }

    stage('Doc') {
      steps {
        // Not sure what to do with this yet
        sh 'cargo doc'
      }
    }
  }
}
